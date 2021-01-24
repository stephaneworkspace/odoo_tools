use super::OdooConnection;
use crate::cfg::HrSelection;
use crate::error::Error;
use crate::error::E_INV_CRED;
use crate::error::E_INV_RESP;
use xmlrpc::{Request, Value};

#[derive(Debug)]
pub struct HrData {
    odoo_connection: OdooConnection,
    hr_selection: HrSelection,
    pub data: Option<HrParse>,
}

#[derive(Debug)]
pub struct HrParse {
    pub section: String,
    pub ligne_note: Vec<(HrLigne, String)>,
}

impl HrParse {
    pub fn new(section: String, ligne_note: Vec<(HrLigne, String)>) -> Self {
        Self {
            section,
            ligne_note,
        }
    }
}

#[derive(Debug)]
pub struct HrLigne {
    pub id: i32,
    pub activity: String,
    pub worked_hours: f64,
}

impl HrLigne {
    pub fn new(id: i32, activity: String, worked_hours: f64) -> Self {
        Self {
            id,
            activity,
            worked_hours,
        }
    }
}

impl HrData {
    pub fn new(odoo_connection: OdooConnection, hr_selection: HrSelection) -> Self {
        Self {
            odoo_connection,
            hr_selection,
            data: None,
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
        vec_select.push(Value::String("worked_hours".to_string()));

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
        let arr = read.as_array().ok_or(E_INV_RESP)?;
        let mut ligne_note: Vec<(HrLigne, String)> = Vec::new();
        for a in arr.to_vec().iter() {
            // Struct
            let s = a.as_struct().ok_or(E_INV_RESP)?;
            // Array
            let employee_id = s["employee_id"].as_array().ok_or(E_INV_RESP)?;
            let mut id: i32 = 0;
            let mut activity: String = "".to_string();
            for (i, e) in employee_id.to_owned().into_iter().enumerate() {
                match i {
                    0 => id = e.as_i32().ok_or(E_INV_RESP)?,
                    1 => activity = e.as_str().ok_or(E_INV_RESP)?.to_string(),
                    _ => {}
                }
            }
            // F64
            let worked_hours = s["worked_hours"].as_f64().ok_or(E_INV_RESP)?;
            let ligne = HrLigne::new(id, activity, worked_hours);
            // String
            let check_in = s["check_in"].as_str().ok_or(E_INV_RESP)?;
            // String
            let check_out = s["check_out"].as_str().ok_or(E_INV_RESP)?;
            let note = format!("{} {}", check_in, check_out);
            ligne_note.push((ligne, note));
        }
        if arr.len() > 0 {
            self.data = Some(HrParse::new(
                self.hr_selection.invoice_date.as_str().to_string(),
                ligne_note,
            ));
        }
        println!("{:?}", self.data);
        Ok(())
    }
}
