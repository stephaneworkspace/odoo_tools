use super::OdooConnection;
use crate::cfg::HrSelection;
use crate::error::Error;
use crate::error::E_INV_CRED;
use crate::error::E_INV_RESP;
use chrono::NaiveDateTime;
use xmlrpc::{Request, Value};

#[derive(Debug)]
pub struct HrData {
    odoo_connection: OdooConnection,
    hr_selection: HrSelection,
    pub data: Option<HrParse>,
}

pub trait Hr {
    fn selection(&mut self) -> Result<(), Error>;
    fn data_to_str(&self) -> String;
}

impl Hr for HrData {
    /// Query select hour in presence odoo
    fn selection(&mut self) -> Result<(), Error> {
        let date_in =
            iso8601::datetime(&self.hr_selection.invoice_date_in.to_string())
                .unwrap();
        let date_out =
            iso8601::datetime(&self.hr_selection.invoice_date_out.to_string())
                .unwrap();
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
                    _ => {},
                }
            }
            // F64
            let worked_hours_f64 =
                s["worked_hours"].as_f64().ok_or(E_INV_RESP)?;
            let worked_hours: f64 = format!("{:.2}", worked_hours_f64)
                .as_str()
                .parse()
                .unwrap_or(worked_hours_f64);

            let ligne = HrLigne::new(id, activity, worked_hours);

            // TODO put in const
            let fmt_date_odoo = "%Y-%m-%d %H:%M:%S";
            let fmt = chrono::format::strftime::StrftimeItems::new("%H:%M");

            // String
            let check_in_str = s["check_in"].as_str().ok_or(E_INV_RESP)?;
            let check_in_ndt_temp: NaiveDateTime =
                chrono::NaiveDateTime::parse_from_str(
                    check_in_str,
                    fmt_date_odoo.clone(),
                )
                .unwrap();
            let check_in_ndt: NaiveDateTime =
                chrono::NaiveDateTime::from_timestamp(
                    check_in_ndt_temp.timestamp() + 3600,
                    0,
                );
            let check_in =
                check_in_ndt.format_with_items(fmt.clone()).to_string();

            // String
            let check_out_str = s["check_out"].as_str().ok_or(E_INV_RESP)?;
            let check_out_ndt_temp = chrono::NaiveDateTime::parse_from_str(
                check_out_str,
                fmt_date_odoo.clone(),
            )
            .unwrap();
            let check_out_ndt: NaiveDateTime =
                chrono::NaiveDateTime::from_timestamp(
                    check_out_ndt_temp.timestamp() + 3600,
                    0,
                );
            let check_out =
                check_out_ndt.format_with_items(fmt.clone()).to_string();

            let note = format!("{} {}", check_in, check_out);
            ligne_note.push((ligne, note));
        }
        ligne_note.reverse();
        if arr.len() > 0 {
            self.data = Some(HrParse::new(
                self.hr_selection.invoice_date.as_str().to_string(),
                ligne_note,
            ));
        }
        Ok(())
    }

    /// Print day output
    fn data_to_str(&self) -> String {
        match self.data.as_ref() {
            Some(data) => {
                let mut vec_string: Vec<String> = Vec::new();
                vec_string.push(data.section.as_str().to_string());
                for (ligne, note) in data.ligne_note.iter() {
                    vec_string.push(format!(
                        "{:<49} {}",
                        ligne.activity, ligne.worked_hours
                    ));
                    vec_string.push(note.as_str().to_string());
                }
                vec_string
                    .iter()
                    .fold(String::new(), |a, b| format!("{}{}\n", a, b))
            },
            None => {
                format!(
                    "Nothing to display for: {}",
                    self.hr_selection.invoice_date
                )
            },
        }
    }
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
    pub fn new(
        odoo_connection: OdooConnection,
        hr_selection: HrSelection,
    ) -> Self {
        Self {
            odoo_connection,
            hr_selection,
            data: None,
        }
    }
}
