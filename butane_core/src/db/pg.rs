//! Postgresql database backend
use super::helper;
use super::*;
use crate::migrations::adb::{AColumn, ATable, Operation, ADB};
use crate::{debug, query};
use crate::{Result, SqlType, SqlVal, SqlValRef};
#[cfg(feature = "datetime")]
use chrono::NaiveDateTime;
use postgres::fallible_iterator::FallibleIterator;
use postgres::GenericClient;
use std::cell::RefCell;
use std::fmt::Write;

#[cfg(feature = "debug")]
use exec_time::exec_time;

/// The name of the postgres backend.
pub const BACKEND_NAME: &str = "pg";

/// Pg [Backend][crate::db::Backend] implementation.
#[derive(Default)]
pub struct PgBackend {}
impl PgBackend {
    pub fn new() -> PgBackend {
        PgBackend {}
    }
}
impl PgBackend {
    fn connect(&self, params: &str) -> Result<PgConnection> {
        PgConnection::open(params)
    }
}
impl Backend for PgBackend {
    fn name(&self) -> &'static str {
        BACKEND_NAME
    }

    fn create_migration_sql(&self, current: &ADB, ops: Vec<Operation>) -> Result<String> {
        let mut current: ADB = (*current).clone();
        Ok(ops
            .iter()
            .map(|o| sql_for_op(&mut current, &o))
            .collect::<Result<Vec<String>>>()?
            .join("\n"))
    }

    fn connect(&self, path: &str) -> Result<Connection> {
        Ok(Connection {
            conn: Box::new(self.connect(path)?),
        })
    }
}

/// Pg database connection.
pub struct PgConnection {
    conn: RefCell<postgres::Client>,
}
impl PgConnection {
    fn open(params: &str) -> Result<Self> {
        Ok(PgConnection {
            conn: RefCell::new(Self::connect(params)?),
        })
    }
    fn connect(params: &str) -> Result<postgres::Client> {
        cfg_if::cfg_if! {
            if #[cfg(feature = "tls")] {
                let connector = native_tls::TlsConnector::new()?;
                let connector = postgres_native_tls::MakeTlsConnector::new(connector);
            } else {
                let connector = postgres::NoTls;
            }
        }
        Ok(postgres::Client::connect(params, connector)?)
    }
}
impl PgConnectionLike for PgConnection {
    type Client = postgres::Client;
    fn cell(&self) -> Result<&RefCell<Self::Client>> {
        Ok(&self.conn)
    }
}
impl BackendConnection for PgConnection {
    fn transaction<'c>(&'c mut self) -> Result<Transaction<'c>> {
        let trans: postgres::Transaction<'_> = self.conn.get_mut().transaction()?;
        let trans = Box::new(PgTransaction::new(trans));
        Ok(Transaction::new(trans))
    }
    fn backend(&self) -> Box<dyn Backend> {
        Box::new(PgBackend {})
    }
    fn backend_name(&self) -> &'static str {
        BACKEND_NAME
    }
    fn is_closed(&self) -> bool {
        self.conn.borrow().is_closed()
    }
}

type DynToSqlPg<'a> = (dyn postgres::types::ToSql + Sync + 'a);

fn sqlval_for_pg_query(v: &SqlVal) -> &dyn postgres::types::ToSql {
    v as &dyn postgres::types::ToSql
}

fn sqlvalref_for_pg_query<'a>(v: &'a SqlValRef<'a>) -> &'a dyn postgres::types::ToSql {
    v as &dyn postgres::types::ToSql
}

/// Shared functionality between connection and
/// transaction. Implementation detail. Semver exempt.
pub trait PgConnectionLike {
    type Client: postgres::GenericClient;
    fn cell(&self) -> Result<&RefCell<Self::Client>>;
}

impl<T> ConnectionMethods for T
where
    T: PgConnectionLike,
{
    fn execute(&self, sql: &str) -> Result<()> {
        if cfg!(feature = "log") {
            debug!("execute sql {}", sql);
        }
        self.cell()?.try_borrow_mut()?.batch_execute(sql.as_ref())?;
        Ok(())
    }

    #[cfg_attr(feature = "debug", exec_time)]
    fn query(
        &self,
        table: &'static str,
        columns: &[Column],
        expr: Option<BoolExpr>,
        limit: Option<i32>,
        order: Option<&[query::Order]>,
    ) -> Result<RawQueryResult> {
        let mut sqlquery = String::new();
        helper::sql_select(columns, table, &mut sqlquery);
        let mut values: Vec<SqlVal> = Vec::new();
        if let Some(expr) = expr {
            sqlquery.write_str(" WHERE ").unwrap();
            sql_for_expr(
                query::Expr::Condition(Box::new(expr)),
                &mut values,
                &mut PgPlaceholderSource::new(),
                &mut sqlquery,
            );
        }

        if let Some(order) = order {
            helper::sql_order(order, &mut sqlquery)
        }

        if let Some(limit) = limit {
            helper::sql_limit(limit, &mut sqlquery)
        }

        if cfg!(feature = "log") {
            debug!("query sql {}", sqlquery);
        }

        let stmt = self.cell()?.try_borrow_mut()?.prepare(&sqlquery)?;
        self.cell()?
            .try_borrow_mut()?
            .query_raw(&stmt, values.iter().map(sqlval_for_pg_query))?
            .map_err(Error::Postgres)
            .map(|r| row_from_postgres(&r, columns))
            .collect()
    }
    fn insert_returning_pk(
        &self,
        table: &'static str,
        columns: &[Column],
        pkcol: &Column,
        values: &[SqlValRef<'_>],
    ) -> Result<SqlVal> {
        let mut sql = String::new();
        helper::sql_insert_with_placeholders(
            table,
            columns,
            &mut PgPlaceholderSource::new(),
            &mut sql,
        );
        write!(&mut sql, " RETURNING {}", pkcol.name()).unwrap();
        if cfg!(feature = "log") {
            debug!("insert sql {}", sql);
        }

        // use query instead of execute so we can get our result back
        let pk: Option<SqlVal> = self
            .cell()?
            .try_borrow_mut()?
            .query_raw(sql.as_str(), values.iter().map(sqlvalref_for_pg_query))?
            .map_err(Error::Postgres)
            .map(|r| sql_val_from_postgres(&r, 0, pkcol))
            .nth(0)?;
        pk.ok_or(Error::Internal)
    }
    fn insert_only(
        &self,
        table: &'static str,
        columns: &[Column],
        values: &[SqlValRef<'_>],
    ) -> Result<()> {
        let mut sql = String::new();
        helper::sql_insert_with_placeholders(
            table,
            columns,
            &mut PgPlaceholderSource::new(),
            &mut sql,
        );
        let params: Vec<&DynToSqlPg> = values.iter().map(|v| v as &DynToSqlPg).collect();
        self.cell()?
            .try_borrow_mut()?
            .execute(sql.as_str(), params.as_slice())?;
        Ok(())
    }
    fn insert_or_replace<'a>(
        &self,
        table: &'static str,
        columns: &[Column],
        pkcol: &Column,
        values: &[SqlValRef<'a>],
    ) -> Result<()> {
        let mut sql = String::new();
        sql_insert_or_replace_with_placeholders(table, columns, pkcol, &mut sql);
        let params: Vec<&DynToSqlPg> = values.iter().map(|v| v as &DynToSqlPg).collect();
        self.cell()?
            .try_borrow_mut()?
            .execute(sql.as_str(), params.as_slice())?;
        Ok(())
    }
    fn update(
        &self,
        table: &'static str,
        pkcol: Column,
        pk: SqlValRef,
        columns: &[Column],
        values: &[SqlValRef<'_>],
    ) -> Result<()> {
        let mut sql = String::new();
        helper::sql_update_with_placeholders(
            table,
            pkcol,
            columns,
            &mut PgPlaceholderSource::new(),
            &mut sql,
        );
        let placeholder_values = [values, &[pk]].concat();
        let params: Vec<&DynToSqlPg> = placeholder_values
            .iter()
            .map(|v| v as &DynToSqlPg)
            .collect();
        if cfg!(feature = "log") {
            debug!("update sql {}", sql);
        }
        self.cell()?
            .try_borrow_mut()?
            .execute(sql.as_str(), params.as_slice())?;
        Ok(())
    }
    fn delete_where(&self, table: &'static str, expr: BoolExpr) -> Result<usize> {
        let mut sql = String::new();
        let mut values: Vec<SqlVal> = Vec::new();
        write!(&mut sql, "DELETE FROM {} WHERE ", table).unwrap();
        sql_for_expr(
            query::Expr::Condition(Box::new(expr)),
            &mut values,
            &mut PgPlaceholderSource::new(),
            &mut sql,
        );
        let params: Vec<&DynToSqlPg> = values.iter().map(|v| v as &DynToSqlPg).collect();
        let cnt = self
            .cell()?
            .try_borrow_mut()?
            .execute(sql.as_str(), params.as_slice())?;
        Ok(cnt as usize)
    }
    fn has_table(&self, table: &'static str) -> Result<bool> {
        // future improvement, should be schema-aware
        let stmt = self
            .cell()?
            .try_borrow_mut()?
            .prepare("SELECT table_name FROM information_schema.tables WHERE table_name=$1;")?;
        let rows = self.cell()?.try_borrow_mut()?.query(&stmt, &[&table])?;
        Ok(!rows.is_empty())
    }
}

struct PgTransaction<'c> {
    trans: Option<RefCell<postgres::Transaction<'c>>>,
}
impl<'c> PgTransaction<'c> {
    fn new(trans: postgres::Transaction<'c>) -> Self {
        PgTransaction {
            trans: Some(RefCell::new(trans)),
        }
    }
    fn get(&self) -> Result<&RefCell<postgres::Transaction<'c>>> {
        match &self.trans {
            None => Err(Error::Internal),
            Some(trans) => Ok(trans),
        }
    }
}
impl<'c> PgConnectionLike for PgTransaction<'c> {
    type Client = postgres::Transaction<'c>;
    fn cell(&self) -> Result<&RefCell<Self::Client>> {
        self.get()
    }
}

impl<'c> BackendTransaction<'c> for PgTransaction<'c> {
    fn commit(&mut self) -> Result<()> {
        match self.trans.take() {
            None => Err(Error::Internal),
            Some(trans) => Ok(trans.into_inner().commit()?),
        }
    }
    fn rollback(&mut self) -> Result<()> {
        match self.trans.take() {
            None => Err(Error::Internal),
            Some(trans) => Ok(trans.into_inner().rollback()?),
        }
    }
    // Workaround for https://github.com/rust-lang/rfcs/issues/2765
    fn connection_methods(&self) -> &dyn ConnectionMethods {
        self
    }
    fn connection_methods_mut(&mut self) -> &mut dyn ConnectionMethods {
        self
    }
}

impl postgres::types::ToSql for SqlVal {
    fn to_sql(
        &self,
        ty: &postgres::types::Type,
        out: &mut bytes::BytesMut,
    ) -> std::result::Result<
        postgres::types::IsNull,
        Box<dyn std::error::Error + 'static + Sync + Send>,
    > {
        self.as_ref().to_sql(ty, out)
    }
    fn accepts(ty: &postgres::types::Type) -> bool {
        SqlValRef::<'_>::accepts(ty)
    }
    postgres::types::to_sql_checked!();
}

impl<'a> postgres::types::ToSql for SqlValRef<'a> {
    fn to_sql(
        &self,
        ty: &postgres::types::Type,
        out: &mut bytes::BytesMut,
    ) -> std::result::Result<
        postgres::types::IsNull,
        Box<dyn std::error::Error + 'static + Sync + Send>,
    > {
        use SqlValRef::*;
        match self {
            Bool(b) => b.to_sql(ty, out),
            Int(i) => i.to_sql(ty, out),
            BigInt(i) => i.to_sql(ty, out),
            Real(r) => r.to_sql(ty, out),
            Text(t) => t.to_sql(ty, out),
            Blob(b) => b.to_sql(ty, out),
            #[cfg(feature = "datetime")]
            Timestamp(dt) => dt.to_sql(ty, out),
            Null => Option::<bool>::None.to_sql(ty, out),
        }
    }

    fn accepts(ty: &postgres::types::Type) -> bool {
        // Unfortunately this is a type method rather than an instance method.
        // Declare acceptance of all the types we can support and hope it works out OK
        if bool::accepts(ty)
            || i32::accepts(ty)
            || i64::accepts(ty)
            || f64::accepts(ty)
            || String::accepts(ty)
            || Vec::<u8>::accepts(ty)
        {
            return true;
        }

        #[cfg(feature = "datetime")]
        if NaiveDateTime::accepts(ty) {
            return true;
        }

        false
    }

    postgres::types::to_sql_checked!();
}

impl<'a> postgres::types::FromSql<'a> for SqlVal {
    fn from_sql(
        ty: &postgres::types::Type,
        raw: &'a [u8],
    ) -> std::result::Result<Self, Box<dyn std::error::Error + 'static + Sync + Send>> {
        use postgres::types::Type;
        match *ty {
            Type::BOOL => Ok(SqlVal::Bool(bool::from_sql(ty, raw)?)),
            Type::INT4 => Ok(SqlVal::Int(i32::from_sql(ty, raw)?)),
            Type::INT8 => Ok(SqlVal::BigInt(i64::from_sql(ty, raw)?)),
            Type::FLOAT8 => Ok(SqlVal::Real(f64::from_sql(ty, raw)?)),
            Type::TEXT => Ok(SqlVal::Text(String::from_sql(ty, raw)?)),
            Type::BYTEA => Ok(SqlVal::Blob(Vec::<u8>::from_sql(ty, raw)?)),
            #[cfg(feature = "datetime")]
            Type::TIMESTAMP => Ok(SqlVal::Timestamp(NaiveDateTime::from_sql(ty, raw)?)),
            _ => Err(Box::new(Error::UnknownSqlType(format!("{}", ty)))),
        }
    }

    fn from_sql_null(
        _ty: &postgres::types::Type,
    ) -> std::result::Result<Self, Box<dyn std::error::Error + 'static + Sync + Send>> {
        Ok(SqlVal::Null)
    }

    #[allow(clippy::match_like_matches_macro)]
    fn accepts(ty: &postgres::types::Type) -> bool {
        use postgres::types::Type;
        match *ty {
            Type::BOOL => true,
            Type::INT2 => true,
            Type::INT4 => true,
            Type::INT8 => true,
            Type::FLOAT4 => true,
            Type::FLOAT8 => true,
            Type::TEXT => true,
            Type::BYTEA => true,
            #[cfg(feature = "datetime")]
            Type::TIMESTAMP => true,
            _ => false,
        }
    }
}

fn row_from_postgres(row: &postgres::Row, cols: &[Column]) -> Result<Row> {
    let mut vals: Vec<SqlVal> = Vec::new();
    if cols.len() != row.len() {
        panic!(
            "postgres returns columns {} doesn't match requested columns {}",
            row.len(),
            cols.len()
        )
    }
    vals.reserve(cols.len());
    for i in 0..cols.len() {
        let col = cols.get(i).unwrap();
        vals.push(sql_val_from_postgres(row, i, col)?);
    }
    Ok(Row::new(vals))
}

fn sql_for_expr<W>(
    expr: query::Expr,
    values: &mut Vec<SqlVal>,
    pls: &mut PgPlaceholderSource,
    w: &mut W,
) where
    W: Write,
{
    helper::sql_for_expr(expr, &sql_for_expr, values, pls, w)
}

fn sql_val_from_postgres<I>(row: &postgres::Row, idx: I, col: &Column) -> Result<SqlVal>
where
    I: postgres::row::RowIndex + std::fmt::Display,
{
    let sqlval: SqlVal = row.try_get(idx)?;
    if sqlval.is_compatible(col.ty(), true) {
        Ok(sqlval)
    } else {
        Err(Error::SqlResultTypeMismatch {
            col: col.name().to_string(),
            detail: format!(
                "{:?} is not compatible with expected column type {}",
                &sqlval,
                col.ty()
            ),
        })
    }
}

fn sql_for_op(current: &mut ADB, op: &Operation) -> Result<String> {
    match op {
        Operation::AddTable(table) => Ok(create_table(&table)?),
        Operation::RemoveTable(name) => Ok(drop_table(&name)),
        Operation::AddColumn(tbl, col) => add_column(&tbl, &col),
        Operation::RemoveColumn(tbl, name) => Ok(remove_column(&tbl, &name)),
        Operation::ChangeColumn(tbl, old, new) => change_column(current, &tbl, &old, Some(new)),
    }
}

fn create_table(table: &ATable) -> Result<String> {
    let coldefs = table
        .columns
        .iter()
        .map(define_column)
        .collect::<Result<Vec<String>>>()?
        .join(",\n");
    Ok(format!("CREATE TABLE {} (\n{}\n);", table.name, coldefs))
}

fn define_column(col: &AColumn) -> Result<String> {
    let mut constraints: Vec<String> = Vec::new();
    if !col.nullable() {
        constraints.push("NOT NULL".to_string());
    }
    if col.is_pk() {
        constraints.push("PRIMARY KEY".to_string());
    }
    Ok(format!(
        "{} {} {}",
        &col.name(),
        col_sqltype(col)?,
        constraints.join(" ")
    ))
}

fn col_sqltype(col: &AColumn) -> Result<&'static str> {
    let ty = col.sqltype()?;
    if col.is_auto() {
        match ty {
            SqlType::Int => Ok("SERIAL"),
            SqlType::BigInt => Ok("BIGSERIAL"),
            _ => Err(Error::InvalidAuto(col.name().to_string())),
        }
    } else {
        Ok(match ty {
            SqlType::Bool => "BOOLEAN",
            SqlType::Int => "INTEGER",
            SqlType::BigInt => "BIGINT",
            SqlType::Real => "DOUBLE PRECISION",
            SqlType::Text => "TEXT",
            #[cfg(feature = "datetime")]
            SqlType::Timestamp => "TIMESTAMP",
            SqlType::Blob => "BYTEA",
        })
    }
}

fn drop_table(name: &str) -> String {
    format!("DROP TABLE {};", name)
}

fn add_column(tbl_name: &str, col: &AColumn) -> Result<String> {
    let default: SqlVal = helper::column_default(col)?;
    Ok(format!(
        "ALTER TABLE {} ADD COLUMN {} DEFAULT {};",
        tbl_name,
        define_column(col)?,
        helper::sql_literal_value(default)
    ))
}

fn remove_column(tbl_name: &str, name: &str) -> String {
    format!("ALTER TABLE {} DROP COLUMN {};", tbl_name, name)
}

fn copy_table(old: &ATable, new: &ATable) -> String {
    let column_names = new
        .columns
        .iter()
        .map(|col| col.name())
        .collect::<Vec<&str>>()
        .join(", ");
    format!(
        "INSERT INTO {} SELECT {} FROM {};",
        &new.name, column_names, &old.name
    )
}

fn tmp_table_name(name: &str) -> String {
    format!("{}__butane_tmp", name)
}

fn change_column(
    current: &mut ADB,
    tbl_name: &str,
    old: &AColumn,
    new: Option<&AColumn>,
) -> Result<String> {
    let table = current.get_table(tbl_name);
    if table.is_none() {
        crate::warn!(
            "Cannot alter column {} from table {} that does not exist",
            &old.name(),
            tbl_name
        );
        return Ok(String::new());
    }
    let old_table = table.unwrap();
    let mut new_table = old_table.clone();
    new_table.name = tmp_table_name(&new_table.name);
    match new {
        Some(col) => new_table.replace_column(col.clone()),
        None => new_table.remove_column(old.name()),
    }
    let stmts: [&str; 4] = [
        &create_table(&new_table)?,
        &copy_table(&old_table, &new_table),
        &drop_table(&old_table.name),
        &format!("ALTER TABLE {} RENAME TO {};", &new_table.name, tbl_name),
    ];
    let result = stmts.join("\n");
    new_table.name = old_table.name.clone();
    current.replace_table(new_table);
    Ok(result)
}

pub fn sql_insert_or_replace_with_placeholders(
    table: &'static str,
    columns: &[Column],
    pkcol: &Column,
    w: &mut impl Write,
) {
    write!(w, "INSERT ").unwrap();
    write!(w, "INTO {} (", table).unwrap();
    helper::list_columns(columns, w);
    write!(w, ") VALUES (").unwrap();
    columns.iter().fold(1, |n, _| {
        let sep = if n == 1 { "" } else { ", " };
        write!(w, "{}${}", sep, n).unwrap();
        n + 1
    });
    write!(w, ")").unwrap();
    write!(w, " ON CONFLICT ({}) DO UPDATE SET (", pkcol.name()).unwrap();
    helper::list_columns(columns, w);
    write!(w, ") = (").unwrap();
    columns.iter().fold("", |sep, c| {
        write!(w, "{}excluded.{}", sep, c.name()).unwrap();
        ", "
    });
    write!(w, ")").unwrap();
}

struct PgPlaceholderSource {
    n: i8,
}
impl PgPlaceholderSource {
    fn new() -> Self {
        PgPlaceholderSource { n: 1 }
    }
}
impl helper::PlaceholderSource for PgPlaceholderSource {
    fn next_placeholder(&mut self) -> Cow<str> {
        let ret = Cow::Owned(format!("${}", self.n));
        self.n += 1;
        ret
    }
}
