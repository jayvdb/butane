#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use butane::db::Connection;
use butane::prelude::*;
use butane::query::BoolExpr;
use butane::{colname, filter, find, query, Many};
use butane_test_helper::*;
#[cfg(feature = "datetime")]
use chrono::{TimeZone, Utc};
mod common {
    pub mod blog {
        use butane::prelude::*;
        use butane::{dataresult, model};
        use butane::{db::Connection, ForeignKey, Many};
        #[cfg(feature = "datetime")]
        use chrono::{naive::NaiveDateTime, offset::Utc};
        pub struct Blog {
            pub id: i64,
            pub name: String,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Blog {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Blog",
                    "id",
                    &self.id,
                    "name",
                    &&self.name,
                )
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for Blog {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<i64>;
                let _: ::core::cmp::AssertParamIsEq<String>;
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Blog {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Blog {
            #[inline]
            fn eq(&self, other: &Blog) -> bool {
                self.id == other.id && self.name == other.name
            }
        }
        impl butane::DataResult for Blog {
            type DBO = Blog;
            const COLUMNS: &'static [butane::db::Column] = &[
                butane::db::Column::new("id", <i64 as butane::FieldType>::SQLTYPE),
                butane::db::Column::new("name", <String as butane::FieldType>::SQLTYPE),
            ];
            fn from_row(mut row: &dyn butane::db::BackendRow) -> butane::Result<Self> {
                if row.len() != 2usize {
                    return Err(
                        butane::Error::BoundsError(
                            "Found unexpected number of columns in row for DataResult"
                                .to_string(),
                        ),
                    );
                }
                let mut obj = Blog {
                    id: butane::FromSql::from_sql_ref(
                        row.get(0usize, <i64 as butane::FieldType>::SQLTYPE)?,
                    )?,
                    name: butane::FromSql::from_sql_ref(
                        row.get(1usize, <String as butane::FieldType>::SQLTYPE)?,
                    )?,
                };
                Ok(obj)
            }
            fn query() -> butane::query::Query<Self> {
                use butane::prelude::DataObject;
                butane::query::Query::new(Self::DBO::TABLE)
            }
        }
        impl butane::internal::DataObjectInternal for Blog {
            const NON_AUTO_COLUMNS: &'static [butane::db::Column] = &[
                butane::db::Column::new("id", <i64 as butane::FieldType>::SQLTYPE),
                butane::db::Column::new("name", <String as butane::FieldType>::SQLTYPE),
            ];
            fn save_many_to_many(
                &self,
                conn: &impl butane::db::ConnectionMethods,
            ) -> butane::Result<()> {
                Ok(())
            }
            fn values(&self, include_pk: bool) -> Vec<butane::SqlValRef> {
                let mut values: Vec<butane::SqlValRef> = Vec::with_capacity(
                    <Self as butane::DataResult>::COLUMNS.len(),
                );
                if (include_pk) {
                    values.push(butane::ToSql::to_sql_ref(&self.id));
                    values.push(butane::ToSql::to_sql_ref(&self.name));
                } else {
                    values.push(butane::ToSql::to_sql_ref(&self.name));
                }
                values
            }
        }
        impl butane::DataObject for Blog {
            type PKType = i64;
            type Fields = BlogFields;
            const PKCOL: &'static str = "id";
            const TABLE: &'static str = "Blog";
            const AUTO_PK: bool = false;
            fn pk(&self) -> &Self::PKType {
                &self.id
            }
        }
        impl butane::ToSql for Blog {
            fn to_sql(&self) -> butane::SqlVal {
                use butane::DataObject;
                butane::ToSql::to_sql(self.pk())
            }
            fn to_sql_ref(&self) -> butane::SqlValRef<'_> {
                use butane::DataObject;
                butane::ToSql::to_sql_ref(self.pk())
            }
        }
        impl butane::ToSql for &Blog {
            fn to_sql(&self) -> butane::SqlVal {
                use butane::DataObject;
                butane::ToSql::to_sql(self.pk())
            }
            fn to_sql_ref(&self) -> butane::SqlValRef<'_> {
                use butane::DataObject;
                butane::ToSql::to_sql_ref(self.pk())
            }
        }
        impl PartialEq<butane::ForeignKey<Blog>> for Blog {
            fn eq(&self, other: &butane::ForeignKey<Blog>) -> bool {
                other.eq(&self)
            }
        }
        impl PartialEq<butane::ForeignKey<Blog>> for &Blog {
            fn eq(&self, other: &butane::ForeignKey<Blog>) -> bool {
                other.eq(self)
            }
        }
        impl butane::AsPrimaryKey<Blog> for Blog {
            fn as_pk(&self) -> std::borrow::Cow<<Self as butane::DataObject>::PKType> {
                use butane::DataObject;
                std::borrow::Cow::Borrowed(self.pk())
            }
        }
        impl butane::AsPrimaryKey<Blog> for &Blog {
            fn as_pk(&self) -> std::borrow::Cow<<Blog as butane::DataObject>::PKType> {
                use butane::DataObject;
                std::borrow::Cow::Borrowed(self.pk())
            }
        }
        impl Blog {
            /// Get fields.
            pub fn fields() -> BlogFields {
                BlogFields::default()
            }
        }
        /// Helper struct for butane model.
        pub struct BlogFields;
        impl BlogFields {
            /// Create query expression.
            pub fn id(&self) -> butane::query::FieldExpr<i64> {
                butane::query::FieldExpr::<i64>::new("id")
            }
            /// Create query expression.
            pub fn name(&self) -> butane::query::FieldExpr<String> {
                butane::query::FieldExpr::<String>::new("name")
            }
        }
        impl std::default::Default for BlogFields {
            fn default() -> Self {
                BlogFields {}
            }
        }
        impl Blog {
            pub fn new(id: i64, name: &str) -> Self {
                Blog { id, name: name.to_string() }
            }
        }
        #[cfg(feature = "datetime")]
        pub struct Post {
            pub id: i64,
            pub title: String,
            pub body: String,
            pub published: bool,
            pub pub_time: std::option::Option<NaiveDateTime>,
            pub likes: i32,
            pub tags: Many<Tag>,
            pub blog: ForeignKey<Blog>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Post {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "id",
                    "title",
                    "body",
                    "published",
                    "pub_time",
                    "likes",
                    "tags",
                    "blog",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.id,
                    &self.title,
                    &self.body,
                    &self.published,
                    &self.pub_time,
                    &self.likes,
                    &self.tags,
                    &&self.blog,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "Post",
                    names,
                    values,
                )
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for Post {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<i64>;
                let _: ::core::cmp::AssertParamIsEq<String>;
                let _: ::core::cmp::AssertParamIsEq<bool>;
                let _: ::core::cmp::AssertParamIsEq<std::option::Option<NaiveDateTime>>;
                let _: ::core::cmp::AssertParamIsEq<i32>;
                let _: ::core::cmp::AssertParamIsEq<Many<Tag>>;
                let _: ::core::cmp::AssertParamIsEq<ForeignKey<Blog>>;
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Post {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Post {
            #[inline]
            fn eq(&self, other: &Post) -> bool {
                self.id == other.id && self.title == other.title
                    && self.body == other.body && self.published == other.published
                    && self.pub_time == other.pub_time && self.likes == other.likes
                    && self.tags == other.tags && self.blog == other.blog
            }
        }
        impl butane::DataResult for Post {
            type DBO = Post;
            const COLUMNS: &'static [butane::db::Column] = &[
                butane::db::Column::new("id", <i64 as butane::FieldType>::SQLTYPE),
                butane::db::Column::new("title", <String as butane::FieldType>::SQLTYPE),
                butane::db::Column::new("body", <String as butane::FieldType>::SQLTYPE),
                butane::db::Column::new(
                    "published",
                    <bool as butane::FieldType>::SQLTYPE,
                ),
                butane::db::Column::new(
                    "pub_time",
                    <std::option::Option<NaiveDateTime> as butane::FieldType>::SQLTYPE,
                ),
                butane::db::Column::new("likes", <i32 as butane::FieldType>::SQLTYPE),
                butane::db::Column::new(
                    "blog",
                    <ForeignKey<Blog> as butane::FieldType>::SQLTYPE,
                ),
            ];
            fn from_row(mut row: &dyn butane::db::BackendRow) -> butane::Result<Self> {
                if row.len() != 7usize {
                    return Err(
                        butane::Error::BoundsError(
                            "Found unexpected number of columns in row for DataResult"
                                .to_string(),
                        ),
                    );
                }
                let mut obj = Post {
                    id: butane::FromSql::from_sql_ref(
                        row.get(0usize, <i64 as butane::FieldType>::SQLTYPE)?,
                    )?,
                    title: butane::FromSql::from_sql_ref(
                        row.get(1usize, <String as butane::FieldType>::SQLTYPE)?,
                    )?,
                    body: butane::FromSql::from_sql_ref(
                        row.get(2usize, <String as butane::FieldType>::SQLTYPE)?,
                    )?,
                    published: butane::FromSql::from_sql_ref(
                        row.get(3usize, <bool as butane::FieldType>::SQLTYPE)?,
                    )?,
                    pub_time: butane::FromSql::from_sql_ref(
                        row
                            .get(
                                4usize,
                                <std::option::Option<
                                    NaiveDateTime,
                                > as butane::FieldType>::SQLTYPE,
                            )?,
                    )?,
                    likes: butane::FromSql::from_sql_ref(
                        row.get(5usize, <i32 as butane::FieldType>::SQLTYPE)?,
                    )?,
                    tags: butane::Many::new(),
                    blog: butane::FromSql::from_sql_ref(
                        row
                            .get(
                                6usize,
                                <ForeignKey<Blog> as butane::FieldType>::SQLTYPE,
                            )?,
                    )?,
                };
                obj.tags
                    .ensure_init(
                        "Post_tags_Many",
                        butane::ToSql::to_sql(obj.pk()),
                        <<Self as butane::DataObject>::PKType as butane::FieldType>::SQLTYPE,
                    );
                Ok(obj)
            }
            fn query() -> butane::query::Query<Self> {
                use butane::prelude::DataObject;
                butane::query::Query::new(Self::DBO::TABLE)
            }
        }
        impl butane::internal::DataObjectInternal for Post {
            const NON_AUTO_COLUMNS: &'static [butane::db::Column] = &[
                butane::db::Column::new("id", <i64 as butane::FieldType>::SQLTYPE),
                butane::db::Column::new("title", <String as butane::FieldType>::SQLTYPE),
                butane::db::Column::new("body", <String as butane::FieldType>::SQLTYPE),
                butane::db::Column::new(
                    "published",
                    <bool as butane::FieldType>::SQLTYPE,
                ),
                butane::db::Column::new(
                    "pub_time",
                    <std::option::Option<NaiveDateTime> as butane::FieldType>::SQLTYPE,
                ),
                butane::db::Column::new("likes", <i32 as butane::FieldType>::SQLTYPE),
                butane::db::Column::new(
                    "blog",
                    <ForeignKey<Blog> as butane::FieldType>::SQLTYPE,
                ),
            ];
            fn save_many_to_many(
                &self,
                conn: &impl butane::db::ConnectionMethods,
            ) -> butane::Result<()> {
                self.tags
                    .ensure_init(
                        "Post_tags_Many",
                        butane::ToSql::to_sql(self.pk()),
                        <<Self as butane::DataObject>::PKType as butane::FieldType>::SQLTYPE,
                    );
                self.tags.save(conn)?;
                Ok(())
            }
            fn values(&self, include_pk: bool) -> Vec<butane::SqlValRef> {
                let mut values: Vec<butane::SqlValRef> = Vec::with_capacity(
                    <Self as butane::DataResult>::COLUMNS.len(),
                );
                if (include_pk) {
                    values.push(butane::ToSql::to_sql_ref(&self.id));
                    values.push(butane::ToSql::to_sql_ref(&self.title));
                    values.push(butane::ToSql::to_sql_ref(&self.body));
                    values.push(butane::ToSql::to_sql_ref(&self.published));
                    values.push(butane::ToSql::to_sql_ref(&self.pub_time));
                    values.push(butane::ToSql::to_sql_ref(&self.likes));
                    values.push(butane::ToSql::to_sql_ref(&self.blog));
                } else {
                    values.push(butane::ToSql::to_sql_ref(&self.title));
                    values.push(butane::ToSql::to_sql_ref(&self.body));
                    values.push(butane::ToSql::to_sql_ref(&self.published));
                    values.push(butane::ToSql::to_sql_ref(&self.pub_time));
                    values.push(butane::ToSql::to_sql_ref(&self.likes));
                    values.push(butane::ToSql::to_sql_ref(&self.blog));
                }
                values
            }
        }
        impl butane::DataObject for Post {
            type PKType = i64;
            type Fields = PostFields;
            const PKCOL: &'static str = "id";
            const TABLE: &'static str = "Post";
            const AUTO_PK: bool = false;
            fn pk(&self) -> &Self::PKType {
                &self.id
            }
        }
        impl butane::ToSql for Post {
            fn to_sql(&self) -> butane::SqlVal {
                use butane::DataObject;
                butane::ToSql::to_sql(self.pk())
            }
            fn to_sql_ref(&self) -> butane::SqlValRef<'_> {
                use butane::DataObject;
                butane::ToSql::to_sql_ref(self.pk())
            }
        }
        impl butane::ToSql for &Post {
            fn to_sql(&self) -> butane::SqlVal {
                use butane::DataObject;
                butane::ToSql::to_sql(self.pk())
            }
            fn to_sql_ref(&self) -> butane::SqlValRef<'_> {
                use butane::DataObject;
                butane::ToSql::to_sql_ref(self.pk())
            }
        }
        impl PartialEq<butane::ForeignKey<Post>> for Post {
            fn eq(&self, other: &butane::ForeignKey<Post>) -> bool {
                other.eq(&self)
            }
        }
        impl PartialEq<butane::ForeignKey<Post>> for &Post {
            fn eq(&self, other: &butane::ForeignKey<Post>) -> bool {
                other.eq(self)
            }
        }
        impl butane::AsPrimaryKey<Post> for Post {
            fn as_pk(&self) -> std::borrow::Cow<<Self as butane::DataObject>::PKType> {
                use butane::DataObject;
                std::borrow::Cow::Borrowed(self.pk())
            }
        }
        impl butane::AsPrimaryKey<Post> for &Post {
            fn as_pk(&self) -> std::borrow::Cow<<Post as butane::DataObject>::PKType> {
                use butane::DataObject;
                std::borrow::Cow::Borrowed(self.pk())
            }
        }
        impl Post {
            /// Get fields.
            pub fn fields() -> PostFields {
                PostFields::default()
            }
        }
        /// Helper struct for butane model.
        pub struct PostFields;
        impl PostFields {
            /// Create query expression.
            pub fn id(&self) -> butane::query::FieldExpr<i64> {
                butane::query::FieldExpr::<i64>::new("id")
            }
            /// Create query expression.
            pub fn title(&self) -> butane::query::FieldExpr<String> {
                butane::query::FieldExpr::<String>::new("title")
            }
            /// Create query expression.
            pub fn body(&self) -> butane::query::FieldExpr<String> {
                butane::query::FieldExpr::<String>::new("body")
            }
            /// Create query expression.
            pub fn published(&self) -> butane::query::FieldExpr<bool> {
                butane::query::FieldExpr::<bool>::new("published")
            }
            /// Create query expression.
            pub fn pub_time(
                &self,
            ) -> butane::query::FieldExpr<std::option::Option<NaiveDateTime>> {
                butane::query::FieldExpr::<
                    std::option::Option<NaiveDateTime>,
                >::new("pub_time")
            }
            /// Create query expression.
            pub fn likes(&self) -> butane::query::FieldExpr<i32> {
                butane::query::FieldExpr::<i32>::new("likes")
            }
            /// Create query expression.
            pub fn tags(&self) -> butane::query::ManyFieldExpr<Post, Tag> {
                butane::query::ManyFieldExpr::<Post, Tag>::new("Post_tags_Many")
            }
            /// Create query expression.
            pub fn blog(&self) -> butane::query::FieldExpr<ForeignKey<Blog>> {
                butane::query::FieldExpr::<ForeignKey<Blog>>::new("blog")
            }
        }
        impl std::default::Default for PostFields {
            fn default() -> Self {
                PostFields {}
            }
        }
        impl Post {
            pub fn new(id: i64, title: &str, body: &str, blog: &Blog) -> Self {
                Post {
                    id,
                    title: title.to_string(),
                    body: body.to_string(),
                    published: false,
                    #[cfg(feature = "datetime")]
                    pub_time: None,
                    likes: 0,
                    tags: Many::new(),
                    blog: ForeignKey::from(blog),
                }
            }
        }
        #[cfg(feature = "datetime")]
        pub struct PostMetadata {
            pub id: i64,
            pub title: String,
            #[cfg(feature = "datetime")]
            pub pub_time: Option<NaiveDateTime>,
        }
        impl butane::DataResult for PostMetadata {
            type DBO = Post;
            const COLUMNS: &'static [butane::db::Column] = &[
                butane::db::Column::new("id", <i64 as butane::FieldType>::SQLTYPE),
                butane::db::Column::new("title", <String as butane::FieldType>::SQLTYPE),
                butane::db::Column::new(
                    "pub_time",
                    <Option<NaiveDateTime> as butane::FieldType>::SQLTYPE,
                ),
            ];
            fn from_row(mut row: &dyn butane::db::BackendRow) -> butane::Result<Self> {
                if row.len() != 3usize {
                    return Err(
                        butane::Error::BoundsError(
                            "Found unexpected number of columns in row for DataResult"
                                .to_string(),
                        ),
                    );
                }
                let mut obj = PostMetadata {
                    id: butane::FromSql::from_sql_ref(
                        row.get(0usize, <i64 as butane::FieldType>::SQLTYPE)?,
                    )?,
                    title: butane::FromSql::from_sql_ref(
                        row.get(1usize, <String as butane::FieldType>::SQLTYPE)?,
                    )?,
                    pub_time: butane::FromSql::from_sql_ref(
                        row
                            .get(
                                2usize,
                                <Option<NaiveDateTime> as butane::FieldType>::SQLTYPE,
                            )?,
                    )?,
                };
                Ok(obj)
            }
            fn query() -> butane::query::Query<Self> {
                use butane::prelude::DataObject;
                butane::query::Query::new(Self::DBO::TABLE)
            }
        }
        pub struct Tag {
            pub tag: String,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Tag {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "Tag",
                    "tag",
                    &&self.tag,
                )
            }
        }
        impl butane::DataResult for Tag {
            type DBO = Tag;
            const COLUMNS: &'static [butane::db::Column] = &[
                butane::db::Column::new("tag", <String as butane::FieldType>::SQLTYPE),
            ];
            fn from_row(mut row: &dyn butane::db::BackendRow) -> butane::Result<Self> {
                if row.len() != 1usize {
                    return Err(
                        butane::Error::BoundsError(
                            "Found unexpected number of columns in row for DataResult"
                                .to_string(),
                        ),
                    );
                }
                let mut obj = Tag {
                    tag: butane::FromSql::from_sql_ref(
                        row.get(0usize, <String as butane::FieldType>::SQLTYPE)?,
                    )?,
                };
                Ok(obj)
            }
            fn query() -> butane::query::Query<Self> {
                use butane::prelude::DataObject;
                butane::query::Query::new(Self::DBO::TABLE)
            }
        }
        impl butane::internal::DataObjectInternal for Tag {
            const NON_AUTO_COLUMNS: &'static [butane::db::Column] = &[
                butane::db::Column::new("tag", <String as butane::FieldType>::SQLTYPE),
            ];
            fn save_many_to_many(
                &self,
                conn: &impl butane::db::ConnectionMethods,
            ) -> butane::Result<()> {
                Ok(())
            }
            fn values(&self, include_pk: bool) -> Vec<butane::SqlValRef> {
                let mut values: Vec<butane::SqlValRef> = Vec::with_capacity(
                    <Self as butane::DataResult>::COLUMNS.len(),
                );
                if (include_pk) {
                    values.push(butane::ToSql::to_sql_ref(&self.tag));
                } else {}
                values
            }
        }
        impl butane::DataObject for Tag {
            type PKType = String;
            type Fields = TagFields;
            const PKCOL: &'static str = "tag";
            const TABLE: &'static str = "tags";
            const AUTO_PK: bool = false;
            fn pk(&self) -> &Self::PKType {
                &self.tag
            }
        }
        impl butane::ToSql for Tag {
            fn to_sql(&self) -> butane::SqlVal {
                use butane::DataObject;
                butane::ToSql::to_sql(self.pk())
            }
            fn to_sql_ref(&self) -> butane::SqlValRef<'_> {
                use butane::DataObject;
                butane::ToSql::to_sql_ref(self.pk())
            }
        }
        impl butane::ToSql for &Tag {
            fn to_sql(&self) -> butane::SqlVal {
                use butane::DataObject;
                butane::ToSql::to_sql(self.pk())
            }
            fn to_sql_ref(&self) -> butane::SqlValRef<'_> {
                use butane::DataObject;
                butane::ToSql::to_sql_ref(self.pk())
            }
        }
        impl PartialEq<butane::ForeignKey<Tag>> for Tag {
            fn eq(&self, other: &butane::ForeignKey<Tag>) -> bool {
                other.eq(&self)
            }
        }
        impl PartialEq<butane::ForeignKey<Tag>> for &Tag {
            fn eq(&self, other: &butane::ForeignKey<Tag>) -> bool {
                other.eq(self)
            }
        }
        impl butane::AsPrimaryKey<Tag> for Tag {
            fn as_pk(&self) -> std::borrow::Cow<<Self as butane::DataObject>::PKType> {
                use butane::DataObject;
                std::borrow::Cow::Borrowed(self.pk())
            }
        }
        impl butane::AsPrimaryKey<Tag> for &Tag {
            fn as_pk(&self) -> std::borrow::Cow<<Tag as butane::DataObject>::PKType> {
                use butane::DataObject;
                std::borrow::Cow::Borrowed(self.pk())
            }
        }
        impl Tag {
            /// Get fields.
            pub fn fields() -> TagFields {
                TagFields::default()
            }
        }
        /// Helper struct for butane model.
        pub struct TagFields;
        impl TagFields {
            /// Create query expression.
            pub fn tag(&self) -> butane::query::FieldExpr<String> {
                butane::query::FieldExpr::<String>::new("tag")
            }
        }
        impl std::default::Default for TagFields {
            fn default() -> Self {
                TagFields {}
            }
        }
        impl Tag {
            pub fn new(tag: &str) -> Self {
                Tag { tag: tag.to_string() }
            }
        }
        pub fn create_tag(conn: &Connection, name: &str) -> Tag {
            let mut tag = Tag::new(name);
            tag.save(conn).unwrap();
            tag
        }
        /// Sets up two blogs
        /// 1. "Cats"
        /// 2. "Mountains"
        #[allow(dead_code)]
        pub fn setup_blog(conn: &Connection) {
            let mut cats_blog = Blog::new(1, "Cats");
            cats_blog.save(conn).unwrap();
            let mut mountains_blog = Blog::new(2, "Mountains");
            mountains_blog.save(conn).unwrap();
            let tag_asia = create_tag(conn, "asia");
            let tag_danger = create_tag(conn, "danger");
            let mut post = Post::new(
                1,
                "The Tiger",
                "The tiger is a cat which would very much like to eat you.",
                &cats_blog,
            );
            post.published = true;
            #[cfg(feature = "datetime")]
            {
                post.pub_time = Some(Utc::now().naive_utc());
            }
            post.likes = 4;
            post.tags.add(&tag_danger).unwrap();
            post.tags.add(&tag_asia).unwrap();
            post.save(conn).unwrap();
            let mut post = Post::new(
                2,
                "Sir Charles",
                "Sir Charles (the Very Second) is a handsome orange gentleman",
                &cats_blog,
            );
            post.published = true;
            post.likes = 20;
            post.save(conn).unwrap();
            let mut post = Post::new(
                3,
                "Mount Doom",
                "You must throw the ring into Mount Doom. Then you get to ride on a cool eagle.",
                &mountains_blog,
            );
            post.published = true;
            post.likes = 10;
            post.tags.add(&tag_danger).unwrap();
            post.save(conn).unwrap();
            let mut post = Post::new(
                4,
                "Mt. Everest",
                "Everest has very little air, and lately it has very many people. This post is unfinished.",
                &mountains_blog,
            );
            post.published = false;
            post.tags.add(&tag_danger).unwrap();
            post.save(conn).unwrap();
        }
    }
}
use common::blog;
use common::blog::{Blog, Post, PostMetadata, Tag};
fn equality(conn: Connection) {
    blog::setup_blog(&conn);
    let mut posts = <Post as butane::DataResult>::query()
        .filter(<Post as butane::DataResult>::DBO::fields().published().eq(&true))
        .load(&conn)
        .unwrap();
    match (&posts.len(), &3) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    posts.sort_by(|p1, p2| p1.id.partial_cmp(&p2.id).unwrap());
    match (&posts[0].title, &"The Tiger") {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&posts[1].title, &"Sir Charles") {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&posts[2].title, &"Mount Doom") {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
fn equality_separate_dataresult(conn: Connection) {
    blog::setup_blog(&conn);
    let mut posts = <PostMetadata as butane::DataResult>::query()
        .filter(
            <PostMetadata as butane::DataResult>::DBO::fields().published().eq(&true),
        )
        .load(&conn)
        .unwrap();
    match (&posts.len(), &3) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    posts.sort_by(|p1, p2| p1.id.partial_cmp(&p2.id).unwrap());
    match (&posts[0].title, &"The Tiger") {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&posts[1].title, &"Sir Charles") {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&posts[2].title, &"Mount Doom") {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
fn ordered(conn: Connection) {
    blog::setup_blog(&conn);
    let posts = <Post as butane::DataResult>::query()
        .filter(<Post as butane::DataResult>::DBO::fields().published().eq(&true))
        .order_asc(Post::fields().title().name())
        .load(&conn)
        .unwrap();
    match (&posts.len(), &3) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&posts[0].title, &"Mount Doom") {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&posts[1].title, &"Sir Charles") {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&posts[2].title, &"The Tiger") {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
fn comparison(conn: Connection) {
    blog::setup_blog(&conn);
    let mut posts = <Post as butane::DataResult>::query()
        .filter(<Post as butane::DataResult>::DBO::fields().likes().lt(&5))
        .load(&conn)
        .unwrap();
    match (&posts.len(), &2) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    posts.sort_by(|p1, p2| p1.id.partial_cmp(&p2.id).unwrap());
    match (&posts[0].title, &"The Tiger") {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&posts[1].title, &"Mt. Everest") {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
fn like(conn: Connection) {
    blog::setup_blog(&conn);
    let mut posts = <Post as butane::DataResult>::query()
        .filter(<Post as butane::DataResult>::DBO::fields().title().like("M%"))
        .load(&conn)
        .unwrap();
    match (&posts.len(), &2) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    posts.sort_by(|p1, p2| p1.id.partial_cmp(&p2.id).unwrap());
    match (&posts[0].title, &"Mount Doom") {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&posts[1].title, &"Mt. Everest") {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
fn combination(conn: Connection) {
    blog::setup_blog(&conn);
    let posts = <Post as butane::DataResult>::query()
        .filter(
            butane::query::BoolExpr::And(
                Box::new(
                    <Post as butane::DataResult>::DBO::fields().published().eq(&true),
                ),
                Box::new(<Post as butane::DataResult>::DBO::fields().likes().lt(&5)),
            ),
        )
        .load(&conn)
        .unwrap();
    match (&posts.len(), &1) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&posts[0].title, &"The Tiger") {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
fn combination_allof(conn: Connection) {
    blog::setup_blog(&conn);
    let posts = Post::query()
        .filter(
            BoolExpr::AllOf(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        <Post as butane::DataResult>::DBO::fields()
                            .published()
                            .eq(&true),
                        <Post as butane::DataResult>::DBO::fields().likes().lt(&5),
                        <Post as butane::DataResult>::DBO::fields()
                            .title()
                            .eq(&"The Tiger"),
                    ]),
                ),
            ),
        )
        .load(&conn)
        .unwrap();
    match (&posts.len(), &1) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&posts[0].title, &"The Tiger") {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
fn not_found(conn: Connection) {
    blog::setup_blog(&conn);
    let posts = <Post as butane::DataResult>::query()
        .filter(
            butane::query::BoolExpr::And(
                Box::new(
                    <Post as butane::DataResult>::DBO::fields().published().eq(&false),
                ),
                Box::new(<Post as butane::DataResult>::DBO::fields().likes().gt(&5)),
            ),
        )
        .load(&conn)
        .unwrap();
    match (&posts.len(), &0) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
fn rustval(conn: Connection) {
    blog::setup_blog(&conn);
    let post = <Post as butane::DataResult>::query()
        .filter(<Post as butane::DataResult>::DBO::fields().title().eq(&"The Tiger"))
        .limit(1)
        .load(&conn)
        .and_then(|mut results| results.pop().ok_or(butane::Error::NoSuchObject))
        .unwrap();
    match (&post.title, &"The Tiger") {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    let f = || "The Tiger";
    let post2 = <Post as butane::DataResult>::query()
        .filter(<Post as butane::DataResult>::DBO::fields().title().eq(&f()))
        .limit(1)
        .load(&conn)
        .and_then(|mut results| results.pop().ok_or(butane::Error::NoSuchObject))
        .unwrap();
    match (&post, &post2) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
fn fkey_match(conn: Connection) {
    blog::setup_blog(&conn);
    let blog: Blog = <Blog as butane::DataResult>::query()
        .filter(<Blog as butane::DataResult>::DBO::fields().name().eq(&"Cats"))
        .limit(1)
        .load(&conn)
        .and_then(|mut results| results.pop().ok_or(butane::Error::NoSuchObject))
        .unwrap();
    let mut posts = <Post as butane::DataResult>::query()
        .filter(<Post as butane::DataResult>::DBO::fields().blog().eq(&&blog))
        .load(&conn)
        .unwrap();
    let posts2 = <Post as butane::DataResult>::query()
        .filter(<Post as butane::DataResult>::DBO::fields().blog().eq(&blog))
        .load(&conn)
        .unwrap();
    let blog_id = blog.id;
    let posts3 = <Post as butane::DataResult>::query()
        .filter(<Post as butane::DataResult>::DBO::fields().blog().eq(&blog_id))
        .load(&conn)
        .unwrap();
    let posts4 = <Post as butane::DataResult>::query()
        .filter(
            <Post as butane::DataResult>::DBO::fields()
                .blog()
                .subfilter(
                    <Post as butane::DataResult>::DBO::fields()
                        .blog()
                        .fields()
                        .name()
                        .eq(&"Cats"),
                ),
        )
        .load(&conn)
        .unwrap();
    match (&posts.len(), &2) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    posts.sort_by(|p1, p2| p1.id.partial_cmp(&p2.id).unwrap());
    match (&posts[0].title, &"The Tiger") {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&posts[1].title, &"Sir Charles") {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&posts, &posts2) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&posts, &posts3) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&posts, &posts4) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
fn many_load(conn: Connection) {
    blog::setup_blog(&conn);
    let post: Post = <Post as butane::DataResult>::query()
        .filter(<Post as butane::DataResult>::DBO::fields().title().eq(&"The Tiger"))
        .limit(1)
        .load(&conn)
        .and_then(|mut results| results.pop().ok_or(butane::Error::NoSuchObject))
        .unwrap();
    let tags = post.tags.load(&conn).unwrap();
    let mut tags: Vec<&Tag> = tags.collect();
    tags.sort_by(|t1, t2| t1.tag.partial_cmp(&t2.tag).unwrap());
    match (&tags[0].tag, &"asia") {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&tags[1].tag, &"danger") {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
fn many_serialize(conn: Connection) {
    blog::setup_blog(&conn);
    let post: Post = <Post as butane::DataResult>::query()
        .filter(<Post as butane::DataResult>::DBO::fields().title().eq(&"The Tiger"))
        .limit(1)
        .load(&conn)
        .and_then(|mut results| results.pop().ok_or(butane::Error::NoSuchObject))
        .unwrap();
    let tags_json: String = serde_json::to_string(&post.tags).unwrap();
    let tags: Many<Tag> = serde_json::from_str(&tags_json).unwrap();
    let tags = tags.load(&conn).unwrap();
    let mut tags: Vec<&Tag> = tags.collect();
    tags.sort_by(|t1, t2| t1.tag.partial_cmp(&t2.tag).unwrap());
    match (&tags[0].tag, &"asia") {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&tags[1].tag, &"danger") {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
fn many_objects_with_tag(conn: Connection) {
    blog::setup_blog(&conn);
    let mut posts = <Post as butane::DataResult>::query()
        .filter(<Post as butane::DataResult>::DBO::fields().tags().containspk("danger"))
        .load(&conn)
        .unwrap();
    posts.sort_by(|p1, p2| p1.id.partial_cmp(&p2.id).unwrap());
    match (&posts[0].title, &"The Tiger") {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&posts[1].title, &"Mount Doom") {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&posts[2].title, &"Mt. Everest") {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
fn many_objects_with_tag_explicit(conn: Connection) {
    blog::setup_blog(&conn);
    let mut posts = <Post as butane::DataResult>::query()
        .filter(
            <Post as butane::DataResult>::DBO::fields()
                .tags()
                .contains(
                    <Post as butane::DataResult>::DBO::fields()
                        .tags()
                        .fields()
                        .tag()
                        .eq(&"danger"),
                ),
        )
        .load(&conn)
        .unwrap();
    posts.sort_by(|p1, p2| p1.id.partial_cmp(&p2.id).unwrap());
    match (&posts[0].title, &"The Tiger") {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&posts[1].title, &"Mount Doom") {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&posts[2].title, &"Mt. Everest") {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
#[cfg(feature = "datetime")]
fn by_timestamp(conn: Connection) {
    blog::setup_blog(&conn);
    let mut post = <Post as butane::DataResult>::query()
        .filter(<Post as butane::DataResult>::DBO::fields().title().eq(&"Sir Charles"))
        .limit(1)
        .load(&conn)
        .and_then(|mut results| results.pop().ok_or(butane::Error::NoSuchObject))
        .unwrap();
    post
        .pub_time = Some(
        Utc.with_ymd_and_hms(1970, 1, 1, 1, 1, 1).single().unwrap().naive_utc(),
    );
    post.save(&conn).unwrap();
    let mut post = <Post as butane::DataResult>::query()
        .filter(<Post as butane::DataResult>::DBO::fields().title().eq(&"The Tiger"))
        .limit(1)
        .load(&conn)
        .and_then(|mut results| results.pop().ok_or(butane::Error::NoSuchObject))
        .unwrap();
    post
        .pub_time = Some(
        Utc.with_ymd_and_hms(1970, 5, 1, 1, 1, 1).single().unwrap().naive_utc(),
    );
    post.save(&conn).unwrap();
    let posts = <Post as butane::DataResult>::query()
        .filter(
            <Post as butane::DataResult>::DBO::fields()
                .pub_time()
                .lt(
                    &Utc
                        .with_ymd_and_hms(1972, 1, 1, 1, 1, 1)
                        .single()
                        .unwrap()
                        .naive_utc(),
                ),
        )
        .order_desc(Post::fields().pub_time().name())
        .load(&conn)
        .unwrap();
    match (&posts[0].title, &"The Tiger") {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&posts[1].title, &"Sir Charles") {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
fn limit(conn: Connection) {
    blog::setup_blog(&conn);
    let posts = Post::query()
        .order_asc(Post::fields().title().name())
        .limit(2)
        .load(&conn)
        .unwrap();
    match (&posts.len(), &2) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&posts[0].title, &"Mount Doom") {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&posts[1].title, &"Mt. Everest") {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
fn offset(conn: Connection) {
    blog::setup_blog(&conn);
    let posts = Post::query()
        .order_asc(Post::fields().title().name())
        .offset(2)
        .load(&conn)
        .unwrap();
    match (&posts.len(), &2) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&posts[0].title, &"Sir Charles") {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&posts[1].title, &"The Tiger") {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
#[rustc_main]
#[coverage(off)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[])
}
