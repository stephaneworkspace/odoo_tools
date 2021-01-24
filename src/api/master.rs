//! https://www.odoo.com/documentation/14.0/webservices/odoo.html
use crate::cfg::Connection;
use crate::error::Error;
use xmlrpc::{Request, Value};

#[derive(Debug)]
pub struct OdooConnection {
    pub connection: Connection,
    pub uid: Option<i32>,
}

impl OdooConnection {
    pub fn new(connection: Connection) -> Self {
        Self {
            connection,
            uid: None,
        }
    }

    /// Login method
    pub fn login(&mut self) -> Result<(), Error> {
        let request_common: String = format!("{}/xmlrpc/2/common", self.connection.url);
        let resp = Request::new("authenticate")
            .arg(self.connection.db.as_str())
            .arg(self.connection.username.as_str())
            .arg(self.connection.password.as_str())
            .arg(Value::Nil)
            .call_url(request_common.as_str())?;
        self.uid = resp.as_i32();
        Ok(())
    }
}
