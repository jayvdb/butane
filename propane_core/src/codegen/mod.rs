use crate::migrations::adb::{DeferredSqlType, TypeKey};
use crate::migrations::{MigrationMut, MigrationsMut};
use crate::{SqlType, SqlVal};
use proc_macro2::TokenStream as TokenStream2;
use proc_macro2::{Ident, Span};
use quote::{quote, ToTokens};
use syn::parse_quote;
use syn::{Attribute, Field, ItemStruct, Lit, LitStr, Meta, MetaNameValue, NestedMeta};

#[macro_export]
macro_rules! make_compile_error {
    ($span:expr=> $($arg:tt)*) => ({
        let lit = crate::codegen::make_lit(&std::fmt::format(format_args!($($arg)*)));
        quote_spanned!($span=> compile_error!(#lit))
    });
    ($($arg:tt)*) => ({
        let lit = crate::codegen::make_lit(&std::fmt::format(format_args!($($arg)*)));
        quote!(compile_error!(#lit))
    })
}

mod dbobj;
mod migration;

pub fn model_with_migrations<M>(
    input: TokenStream2,
    ms: &mut impl MigrationsMut<M = M>,
) -> TokenStream2
where
    M: MigrationMut,
{
    // Transform into a derive because derives can have helper
    // attributes but proc macro attributes can't yet (nor can they
    // create field attributes)
    let mut ast_struct: ItemStruct = syn::parse2(input).unwrap();
    let mut config = dbobj::Config::default();
    for attr in &ast_struct.attrs {
        if let Ok(Meta::NameValue(MetaNameValue {
            path,
            lit: Lit::Str(s),
            ..
        })) = attr.parse_meta()
        {
            if path.is_ident("table") {
                config.table_name = Some(s.value())
            }
        }
    }
    // Filter out our helper attributes
    let attrs: Vec<Attribute> = ast_struct
        .attrs
        .clone()
        .into_iter()
        .filter(|a| !a.path.is_ident("table"))
        .collect();

    let state_attrs = if has_derive_serialize(&attrs) {
        quote!(#[serde(skip)])
    } else {
        TokenStream2::new()
    };

    let vis = &ast_struct.vis;

    migration::write_table_to_disk(ms, &ast_struct, &config).unwrap();

    let impltraits = dbobj::impl_dbobject(&ast_struct, &config);
    let fieldexprs = dbobj::add_fieldexprs(&ast_struct);

    match &mut ast_struct.fields {
        syn::Fields::Named(fields) => {
            for field in &mut fields.named {
                field.attrs.retain(|a| {
                    !a.path.is_ident("pk")
                        && !a.path.is_ident("auto")
                        && !a.path.is_ident("sqltype")
                        && !a.path.is_ident("default")
                });
            }
        }
        _ => return make_compile_error!("Fields must be named").into(),
    };
    let fields = match ast_struct.fields {
        syn::Fields::Named(fields) => fields.named,
        _ => return make_compile_error!("Fields must be named").into(),
    };

    let ident = ast_struct.ident;

    quote!(
        #(#attrs)*
        #vis struct #ident {
            #state_attrs
            pub state: propane::ObjectState,
            #fields
        }
        #impltraits
        #fieldexprs
    )
    .into()
}

fn make_ident_literal_str(ident: &Ident) -> LitStr {
    let as_str = format!("{}", ident);
    LitStr::new(&as_str, Span::call_site())
}

pub fn make_lit(s: &str) -> LitStr {
    LitStr::new(s, Span::call_site())
}

fn pk_field(ast_struct: &ItemStruct) -> Option<Field> {
    let pk_by_attribute =
        fields(ast_struct).find(|f| f.attrs.iter().any(|attr| attr.path.is_ident("pk")));
    if let Some(id_field) = pk_by_attribute {
        return Some(id_field.clone());
    }
    let pk_by_name = ast_struct.fields.iter().find(|f| match &f.ident {
        Some(ident) => *ident == "id",
        None => false,
    });
    if let Some(id_field) = pk_by_name {
        Some(id_field.clone())
    } else {
        None
    }
}

fn is_auto(field: &Field) -> bool {
    field.attrs.iter().any(|attr| attr.path.is_ident("auto"))
}

fn fields(ast_struct: &ItemStruct) -> impl Iterator<Item = &Field> {
    ast_struct
        .fields
        .iter()
        .filter(|f| f.ident.clone().unwrap() != "state")
}

fn get_option_sql_type(ty: &syn::Type) -> Option<DeferredSqlType> {
    get_foreign_type_argument(ty, "Option").map(|path| {
        let inner_ty: syn::Type = syn::TypePath {
            qself: None,
            path: path.clone(),
        }
        .into();

        get_deferred_sql_type(&inner_ty)
    })
}

fn get_many_sql_type(field: &Field) -> Option<DeferredSqlType> {
    get_foreign_sql_type(&field.ty, "Many")
}

fn is_many_to_many(field: &Field) -> bool {
    get_many_sql_type(field).is_some()
}

fn is_option(field: &Field) -> bool {
    get_foreign_type_argument(&field.ty, "Option").is_some()
}

/// Check for special fields which won't correspond to rows and don't
/// implement FieldType
fn is_row_field(f: &Field) -> bool {
    !is_many_to_many(f)
}

fn get_foreign_type_argument<'a>(ty: &'a syn::Type, tyname: &'static str) -> Option<&'a syn::Path> {
    let path = match ty {
        syn::Type::Path(path) => &path.path,
        _ => return None,
    };
    let seg = if path.segments.len() == 2 && path.segments.first().unwrap().ident == "propane" {
        path.segments.last()
    } else {
        path.segments.first()
    }?;
    if seg.ident != tyname {
        return None;
    }
    let args = match &seg.arguments {
        syn::PathArguments::AngleBracketed(args) => &args.args,
        _ => return None,
    };
    if args.len() != 1 {
        panic!("{} should have a single type argument", tyname)
    }
    match args.last().unwrap() {
        syn::GenericArgument::Type(syn::Type::Path(typath)) => Some(&typath.path),
        _ => panic!("{} argument should be a type.", tyname),
    }
}

fn get_foreign_sql_type(ty: &syn::Type, tyname: &'static str) -> Option<DeferredSqlType> {
    let typath = get_foreign_type_argument(ty, tyname);
    typath.map(|typath| {
        DeferredSqlType::Deferred(TypeKey::PK(
            typath
                .segments
                .last()
                .unwrap_or_else(|| panic!("{} must have an argument", tyname))
                .ident
                .to_string(),
        ))
    })
}

pub fn get_deferred_sql_type(ty: &syn::Type) -> DeferredSqlType {
    get_primitive_sql_type(ty)
        .or_else(|| get_option_sql_type(ty))
        .or_else(|| get_foreign_sql_type(ty, "ForeignKey"))
        .unwrap_or_else(|| {
            DeferredSqlType::Deferred(TypeKey::CustomType(
                ty.clone().into_token_stream().to_string(),
            ))
        })
}

/// Defaults are used for fields added by later migrations
/// Example
/// #[default = 42]
fn get_default(field: &Field) -> std::result::Result<Option<SqlVal>, CompilerErrorMsg> {
    let attr: Option<&Attribute> = field
        .attrs
        .iter()
        .find(|attr| attr.path.is_ident("default"));
    let lit: Lit = match attr {
        None => return Ok(None),
        Some(attr) => match attr.parse_meta() {
            Ok(Meta::NameValue(meta)) => meta.lit,
            _ => return Err(make_compile_error!("malformed default value").into()),
        },
    };
    Ok(Some(sqlval_from_lit(lit)?))
}

/// If the field refers to a primitive, return its SqlType
fn get_primitive_sql_type(ty: &syn::Type) -> Option<DeferredSqlType> {
    if *ty == parse_quote!(bool) {
        return Some(DeferredSqlType::Known(SqlType::Bool));
    } else if *ty == parse_quote!(u8)
        || *ty == parse_quote!(i8)
        || *ty == parse_quote!(u16)
        || *ty == parse_quote!(i16)
        || *ty == parse_quote!(u16)
        || *ty == parse_quote!(i32)
    {
        return Some(DeferredSqlType::Known(SqlType::Int));
    } else if *ty == parse_quote!(u32) || *ty == parse_quote!(i64) {
        // TODO better support unsigned integers here. Sqlite has no u64, though Postgres does
        return Some(DeferredSqlType::Known(SqlType::BigInt));
    } else if *ty == parse_quote!(f32) || *ty == parse_quote!(f64) {
        return Some(DeferredSqlType::Known(SqlType::Real));
    } else if *ty == parse_quote!(String) {
        return Some(DeferredSqlType::Known(SqlType::Text));
    } else if *ty == parse_quote!(Vec<u8>) {
        return Some(DeferredSqlType::Known(SqlType::Blob));
    }

    #[cfg(feature = "datetime")]
    {
        if *ty == parse_quote!(NaiveDateTime) {
            return Some(DeferredSqlType::Known(SqlType::Timestamp));
        }
    }

    #[cfg(feature = "uuid")]
    {
        if *ty == parse_quote!(Uuid) || *ty == parse_quote!(uuid::Uuid) {
            return Some(DeferredSqlType::Known(SqlType::Blob));
        }
    }

    None
}

fn has_derive_serialize(attrs: &[Attribute]) -> bool {
    for attr in attrs {
        if let Ok(Meta::List(ml)) = attr.parse_meta() {
            if ml.path.is_ident("derive")
                && ml.nested.iter().any(|nm| match nm {
                    NestedMeta::Meta(Meta::Path(path)) => path.is_ident("Serialize"),
                    _ => false,
                })
            {
                return true;
            }
        }
    }
    false
}

fn sqlval_from_lit(lit: Lit) -> std::result::Result<SqlVal, CompilerErrorMsg> {
    match lit {
        Lit::Str(lit) => Ok(SqlVal::Text(lit.value())),
        Lit::ByteStr(lit) => Ok(SqlVal::Blob(lit.value())),
        Lit::Byte(_) => Err(make_compile_error!("single byte literal is not supported").into()),
        Lit::Char(_) => Err(make_compile_error!("single char literal is not supported").into()),
        Lit::Int(lit) => Ok(SqlVal::Int(lit.base10_parse().unwrap())),
        Lit::Float(lit) => Ok(SqlVal::Real(lit.base10_parse().unwrap())),
        Lit::Bool(lit) => Ok(SqlVal::Bool(lit.value)),
        Lit::Verbatim(_) => {
            Err(make_compile_error!("raw verbatim literals are not supported").into())
        }
    }
}

#[derive(Debug)]
struct CompilerErrorMsg {
    ts: TokenStream2,
}
impl CompilerErrorMsg {
    fn new(ts: TokenStream2) -> Self {
        CompilerErrorMsg { ts }
    }
}
impl From<TokenStream2> for CompilerErrorMsg {
    fn from(ts: TokenStream2) -> Self {
        CompilerErrorMsg::new(ts)
    }
}
