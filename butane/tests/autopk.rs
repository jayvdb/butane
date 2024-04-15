#![allow(clippy::disallowed_names)]

use butane::db::Connection;
use butane::{butane_type, find, model, query, AutoPk, ForeignKey};
use butane::{colname, prelude::*};
use butane_test_helper::*;
#[cfg(feature = "datetime")]
use chrono::{naive::NaiveDateTime, offset::Utc, DateTime};
use serde::Serialize;

#[model]
struct Baz {
    id: AutoPk<i64>,
    text: String,
}
impl Baz {
    fn new(text: &str) -> Self {
        Baz {
            id: AutoPk::default(),
            text: text.to_string(),
        }
    }
}

fn auto_pk(conn: Connection) {
    let mut baz1 = Baz::new("baz1");
    baz1.save(&conn).unwrap();
    let mut baz2 = Baz::new("baz2");
    baz2.save(&conn).unwrap();
    let mut baz3 = Baz::new("baz3");
    baz3.save(&conn).unwrap();
    assert!(baz1.id < baz2.id);
    assert!(baz2.id < baz3.id);
}
testall!(auto_pk);


#[model]
#[derive(Default)]
struct HasOnlyAutoPk {
    id: AutoPk<i64>,
}

fn only_auto_pk(conn: Connection) {
    let mut obj = HasOnlyAutoPk::default();
    obj.save(&conn).unwrap();
    let pk = obj.id;
    // verify we can still save the object even though it has no
    // fields to modify
    obj.save(&conn).unwrap();
    // verify it didnt get a new id
    assert_eq!(obj.id, pk);
}
testall!(only_auto_pk);
