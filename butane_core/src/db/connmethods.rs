//! Not expected to be called directly by most users. Used by code
//! generated by `#[model]`, `query!`, and other macros.

use crate::query::{BoolExpr, Expr, Order};
use crate::{Result, SqlType, SqlVal, SqlValRef};
use std::ops::{Deref, DerefMut};
use std::vec::Vec;
use async_trait::async_trait;

/// Methods available on a database connection. Most users do not need
/// to call these methods directly and will instead use methods on
/// [DataObject][crate::DataObject] or the `query!` macro. This trait is
/// implemented by both database connections and transactions.
#[async_trait]
pub trait ConnectionMethods: Sync {
    fn execute(&self, sql: &str) -> Result<()>;
    async fn query<'a, 'b, 'c: 'a>(
        &'c self,
        table: &str,
        columns: &'b [Column],
        expr: Option<BoolExpr>,
        limit: Option<i32>,
        offset: Option<i32>,
        sort: Option<&[Order]>,
    ) -> Result<RawQueryResult<'a>>;
    fn insert_returning_pk(
        &self,
        table: &str,
        columns: &[Column],
        pkcol: &Column,
        values: &[SqlValRef<'_>],
    ) -> Result<SqlVal>;
    /// Like `insert_returning_pk` but with no return value
    fn insert_only(&self, table: &str, columns: &[Column], values: &[SqlValRef<'_>]) -> Result<()>;
    /// Insert unless there's a conflict on the primary key column, in which case update
    fn insert_or_replace(
        &self,
        table: &str,
        columns: &[Column],
        pkcol: &Column,
        values: &[SqlValRef<'_>],
    ) -> Result<()>;
    fn update(
        &self,
        table: &str,
        pkcol: Column,
        pk: SqlValRef,
        columns: &[Column],
        values: &[SqlValRef<'_>],
    ) -> Result<()>;
    fn delete(&self, table: &str, pkcol: &'static str, pk: SqlVal) -> Result<()> {
        self.delete_where(table, BoolExpr::Eq(pkcol, Expr::Val(pk)))?;
        Ok(())
    }
    fn delete_where(&self, table: &str, expr: BoolExpr) -> Result<usize>;
    /// Tests if a table exists in the database.
    fn has_table(&self, table: &str) -> Result<bool>;
}

/// Represents a database column. Most users do not need to use this
/// directly.
pub struct Column {
    name: &'static str,
    ty: SqlType,
}
impl Column {
    pub const fn new(name: &'static str, ty: SqlType) -> Self {
        Column { name, ty }
    }
    pub fn name(&self) -> &'static str {
        self.name
    }
    pub fn ty(&self) -> &SqlType {
        &self.ty
    }
}

/// Backend-specific row abstraction. Only implementors of new
/// backends need use this trait directly.
pub trait BackendRow {
    fn get(&self, idx: usize, ty: SqlType) -> Result<SqlValRef>;
    fn len(&self) -> usize;
    // clippy wants this method to exist
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Abstraction of rows returned from a query. Most users do not need
/// to deal with this directly and should use the `query!` macro or
/// [Query](crate::query::Query) type.
pub trait BackendRows {
    // Advance to the next item and get it
    fn next<'a>(&'a mut self) -> Result<Option<&'a (dyn BackendRow + 'a)>>;
    // Get the item most recently returned by next
    fn current<'a>(&'a self) -> Option<&'a (dyn BackendRow + 'a)>;
    #[inline]
    fn mapped<F, B>(self, f: F) -> MapDeref<Self, F>
    where
        Self: Sized,
        F: FnMut(&(dyn BackendRow)) -> Result<B>,
    {
        MapDeref { it: self, f }
    }
}

#[derive(Debug)]
pub struct MapDeref<I, F> {
    it: I,
    f: F,
}

impl<I, F, B> fallible_iterator::FallibleIterator for MapDeref<I, F>
where
    I: BackendRows,
    F: FnMut(&(dyn BackendRow)) -> Result<B>,
{
    type Item = B;
    type Error = crate::Error;

    #[inline]
    fn next(&mut self) -> Result<Option<Self::Item>> {
        match self.it.next() {
            Ok(Some(v)) => (self.f)(v).map(Some),
            Ok(None) => Ok(None),
            Err(e) => Err(e),
        }
    }
}

pub type RawQueryResult<'a> = Box<dyn BackendRows + 'a>;
pub type QueryResult<T> = Vec<T>;

pub trait ConnectionMethodWrapper {
    type Wrapped: ConnectionMethods;
    fn wrapped_connection_methods(&self) -> Result<&Self::Wrapped>;
}

pub(crate) struct VecRows<T> {
    rows: Vec<T>,
    idx: usize,
}
impl<T> VecRows<T> {
    #[allow(unused)] // Not used with all feature combinations
    pub fn new(rows: Vec<T>) -> Self {
        VecRows { rows, idx: 0 }
    }
}
impl<T> BackendRows for VecRows<T>
where
    T: BackendRow,
{
    fn next(&mut self) -> Result<Option<&(dyn BackendRow)>> {
        let ret = self.rows.get(self.idx);
        self.idx += 1;
        Ok(ret.map(|row| row as &dyn BackendRow))
    }

    fn current(&self) -> Option<&(dyn BackendRow)> {
        self.rows.get(self.idx).map(|row| row as &dyn BackendRow)
    }
}

impl<'a> BackendRows for Box<dyn BackendRows + 'a> {
    fn next(&mut self) -> Result<Option<&(dyn BackendRow)>> {
        BackendRows::next(self.deref_mut())
    }

    fn current(&self) -> Option<&(dyn BackendRow)> {
        self.deref().current()
    }
}
