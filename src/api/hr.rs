use super::OdooConnection;
use crate::cfg::HrSelection;
use crate::error::Error;
use crate::error::E_INV_CRED;
use crate::error::E_INV_RESP;
use std::collections::BTreeMap;
use xmlrpc::{Request, Value};
// To simplify definitions using the XML-RPC "struct" type
type OdooDataMap = BTreeMap<String, Value>;

pub struct HrData {
    odoo_connection: OdooConnection,
    hr_selection: HrSelection,
    pub value: Value,
    //pub map: OdooDataMap,
}

impl HrData {
    pub fn new(odoo_connection: OdooConnection, hr_selection: HrSelection) -> Self {
        Self {
            odoo_connection,
            hr_selection,
            value: Value::Nil,
            //map: OdooDataMap::new(),
        }
    }
}

pub trait Hr {
    fn selection(&mut self) -> Result<(), Error>;
}

impl Hr for HrData {
    fn selection(&mut self) -> Result<(), Error> {
        let date_in = iso8601::datetime(&self.hr_selection.invoice_date_in.to_string()).unwrap();
        let date_out = iso8601::datetime(&self.hr_selection.invoice_date_out.to_string()).unwrap();
        let request_object: String =
            format!("{}/xmlrpc/2/object", self.odoo_connection.connection.url);

        // Read key
        let mut vec_select: Vec<Value> = Vec::new();
        vec_select.push(Value::String("employee_id".to_string()));
        vec_select.push(Value::String("check_in".to_string()));
        vec_select.push(Value::String("check_out".to_string()));

        let mut vec_read1: Vec<Value> = Vec::new();
        let mut vec_read2: Vec<Value> = Vec::new();

        let mut vec_read3: Vec<Value> = Vec::new();
        vec_read3.push(Value::String("check_in".to_string()));
        vec_read3.push(Value::String(">=".to_string()));
        vec_read3.push(Value::DateTime(date_in));
        vec_read2.push(Value::Array(vec_read3));

        let mut vec_read3: Vec<Value> = Vec::new();
        vec_read3.push(Value::String("check_out".to_string()));
        vec_read3.push(Value::String("<=".to_string()));
        vec_read3.push(Value::DateTime(date_out));
        vec_read2.push(Value::Array(vec_read3));

        vec_read1.push(Value::Array(vec_read2));

        let read = Request::new("execute_kw")
            .arg(self.odoo_connection.connection.db.as_str())
            .arg(self.odoo_connection.uid.ok_or(E_INV_CRED)?)
            .arg(self.odoo_connection.connection.password.as_str())
            .arg("hr.attendance")
            .arg("search_read")
            .arg(Value::Array(vec_read1))
            .arg(Value::Struct(
                vec![("fields".to_string(), Value::Array(vec_select))]
                    .into_iter()
                    .collect(),
            ))
            .call_url(request_object.as_str())?;
        self.value = read.clone();
        let arr = read.as_array().ok_or(E_INV_RESP)?;
        for a in arr.to_vec().iter() {
            let s = a.as_struct().ok_or(E_INV_RESP)?;
            let employee_id = s["employee_id"].as_array().ok_or(E_INV_RESP);
            let check_in = s["check_in"].as_str().ok_or(E_INV_RESP);
            let check_out = s["check_out"].as_str().ok_or(E_INV_RESP);
            println!("{:?}", employee_id);
            println!("{:?}", check_in);
            println!("{:?}", check_out);
        }
        //let resp = read.as_struct().ok_or(E_INV_RESP)?;
        Ok(())
    }
}
