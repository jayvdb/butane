use std::ops::Deref;

use super::connmethods::{RawQueryResult};

use super::connmethods::sync::ConnectionMethodWrapper;
use super::sync::{
    Connection, Backend, BackendConnection, BackendTransaction, ConnectionMethods,
    
};
use crate::db::ConnectionSpec;
//use crate::connection_method_wrapper;

use crate::{Column, query::BoolExpr, Error, Result, SqlVal, SqlValRef};

pub use r2d2::ManageConnection;
pub use r2d2::PooledConnection;
pub use r2d2::PooledConnection as PooledConnectionSync;
//pub use r2d2::PooledConnection as PooledConnectionAsync;

/// R2D2 support for Butane. Implements [`r2d2::ManageConnection`].
#[derive(Clone, Debug)]
pub struct ConnectionManager {
    spec: ConnectionSpec,
}
impl ConnectionManager {
    pub fn new(spec: ConnectionSpec) -> Self {
        ConnectionManager { spec }
    }
}

impl ManageConnection for ConnectionManager {
    type Connection = Connection;
    type Error = crate::Error;

    fn connect(&self) -> Result<Self::Connection> {
        //crate::db::connect(&self.spec)
        super::get_backend_sync(&self.spec.backend_name)
        .ok_or_else(|| Error::UnknownBackend(self.spec.backend_name.clone()))?
        .connect(&self.spec.conn_str)
    }

    fn is_valid(&self, conn: &mut Self::Connection) -> Result<()> {
        conn.execute("SELECT 1")
    }

    fn has_broken(&self, conn: &mut Self::Connection) -> bool {
        conn.is_closed()
    }
}

impl ConnectionMethodWrapper for PooledConnection<ConnectionManager> {
    type Wrapped = Connection;
    fn wrapped_connection_methods(&self) -> Result<&Connection> {
        Ok(self.deref())
    }
}

//connection_method_wrapper!(PooledConnection<ConnectionManager>);
