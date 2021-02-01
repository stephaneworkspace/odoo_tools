use crate::api::OdooConnection;
use crate::error::{Error, E_INV_CRED, E_INV_RESP};
use xmlrpc::{Request, Value};

#[derive(Debug)]
pub struct InvoiceData {
    odoo_connection: OdooConnection,
    pub state: String,
}

pub trait Invoice {
    fn selection(&mut self) -> Result<(), Error>;
}

impl Invoice for InvoiceData {
    /// Query invoice
    fn selection(&mut self) -> Result<(), Error> {
        let request_object: String =
            format!("{}/xmlrpc/2/object", self.odoo_connection.connection.url);
        // Read key
        let mut vec_select: Vec<Value> = Vec::new();
        vec_select.push(Value::String("name".to_string()));
        vec_select.push(Value::String("date".to_string()));
        vec_select.push(Value::String("invoice_partner_display_name".to_string()));
        vec_select.push(Value::String("state".to_string()));
        let mut vec_read1: Vec<Value> = Vec::new();
        let mut vec_read2: Vec<Value> = Vec::new();
        let mut vec_read3: Vec<Value> = Vec::new();
        if self.state == "sent".to_string() {
            vec_read3.push(Value::String("state".to_string()));
            vec_read3.push(Value::String("=".to_string()));
            vec_read3.push(Value::String("sent".to_string())); // sent posted and ???
        } else {
            vec_read3.push(Value::String("state".to_string()));
            vec_read3.push(Value::String("!=".to_string()));
            vec_read3.push(Value::String("sent".to_string()));
        }
        vec_read2.push(Value::Array(vec_read3));
        vec_read1.push(Value::Array(vec_read2));
        let read = Request::new("execute_kw")
            .arg(self.odoo_connection.connection.db.as_str())
            .arg(self.odoo_connection.uid.ok_or(E_INV_CRED)?)
            .arg(self.odoo_connection.connection.password.as_str())
            .arg("account.move")
            .arg("search_read")
            .arg(Value::Array(vec_read1))
            .arg(Value::Struct(
                vec![("fields".to_string(), Value::Array(vec_select))]
                    .into_iter()
                    .collect(),
            ))
            .call_url(request_object.as_str())?;
        let arr = read.as_array().ok_or(E_INV_RESP)?;
        println!("{:?}", arr);
        Ok(())
    }
}

impl InvoiceData {
    pub fn new(
        odoo_connection: OdooConnection,
        state: String,
    ) -> Self {
        Self {
            odoo_connection,
            state,
        }
    }
}