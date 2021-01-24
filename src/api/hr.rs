use super::OdooConnection;
use crate::cfg::HrSelection;
use crate::error::Error;
use crate::error::E_INV_CRED;
use crate::error::E_INV_RESP;
use crate::odoo_const::FMT_DATE_INVOICE;
use crate::odoo_const::FMT_DATE_ODOO;
use crate::odoo_const::PRODUCT_PRODUCT_ID_UNKNOWN;
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

            // Read product_template -> id
            let mut vec_select: Vec<Value> = Vec::new();
            vec_select.push(Value::String("id".to_string()));

            let mut vec_read1: Vec<Value> = Vec::new();
            let mut vec_read2: Vec<Value> = Vec::new();

            let mut vec_read3: Vec<Value> = Vec::new();
            vec_read3.push(Value::String("default_code".to_string()));
            vec_read3.push(Value::String("=".to_string()));
            vec_read3.push(Value::String(activity.clone()));
            vec_read2.push(Value::Array(vec_read3));

            vec_read1.push(Value::Array(vec_read2));

            let read_product = Request::new("execute_kw")
                .arg(self.odoo_connection.connection.db.as_str())
                .arg(self.odoo_connection.uid.ok_or(E_INV_CRED)?)
                .arg(self.odoo_connection.connection.password.as_str())
                .arg("product.product")
                .arg("search_read")
                .arg(Value::Array(vec_read1))
                .arg(Value::Struct(
                    vec![("fields".to_string(), Value::Array(vec_select))]
                        .into_iter()
                        .collect(),
                ))
                .call_url(request_object.as_str())?;
            let mut product_id: Option<i32> = None;
            for (_, a_p_id) in read_product
                .as_array()
                .ok_or(E_INV_RESP)?
                .to_vec()
                .iter()
                .enumerate()
                .filter(|(i, _)| i == &0)
            {
                let s = a_p_id.as_struct().ok_or(E_INV_RESP)?;
                // Array
                product_id = Some(s["id"].as_i32().ok_or(E_INV_RESP)?);
                break;
            }

            // Read fields product.template
            let mut vec_select: Vec<Value> = Vec::new();
            vec_select.push(Value::String("name".to_string()));
            vec_select.push(Value::String("description_sale".to_string()));
            vec_select.push(Value::String("list_price".to_string()));

            let mut vec_read1: Vec<Value> = Vec::new();
            let mut vec_read2: Vec<Value> = Vec::new();

            let mut vec_read3: Vec<Value> = Vec::new();
            vec_read3.push(Value::String("id".to_string()));
            vec_read3.push(Value::String("=".to_string()));
            vec_read3.push(Value::Int(
                product_id.unwrap_or(PRODUCT_PRODUCT_ID_UNKNOWN),
            ));
            vec_read2.push(Value::Array(vec_read3));

            vec_read1.push(Value::Array(vec_read2));

            let read_template = Request::new("execute_kw")
                .arg(self.odoo_connection.connection.db.as_str())
                .arg(self.odoo_connection.uid.ok_or(E_INV_CRED)?)
                .arg(self.odoo_connection.connection.password.as_str())
                .arg("product.template")
                .arg("search_read")
                .arg(Value::Array(vec_read1))
                .arg(Value::Struct(
                    vec![("fields".to_string(), Value::Array(vec_select))]
                        .into_iter()
                        .collect(),
                ))
                .call_url(request_object.as_str())?;
            let mut product_name: Option<String> = None;
            let mut product_description_sale: Option<String> = None;
            let mut product_list_price: Option<f64> = None;
            for (_, template) in read_template
                .as_array()
                .ok_or(E_INV_RESP)?
                .to_vec()
                .iter()
                .enumerate()
                .filter(|(i, _)| i == &0)
            {
                let s = template.as_struct().ok_or(E_INV_RESP)?;
                // Array
                product_name = match product_id {
                    Some(p_id) => {
                        if p_id != PRODUCT_PRODUCT_ID_UNKNOWN {
                            Some(
                                s["name"]
                                    .as_str()
                                    .ok_or(E_INV_RESP)?
                                    .to_string(),
                            )
                        } else {
                            None
                        }
                    },
                    None => None,
                };
                product_description_sale = Some(
                    s["description_sale"]
                        .as_str()
                        .ok_or(E_INV_RESP)?
                        .to_string(),
                );
                product_list_price =
                    Some(s["list_price"].as_f64().ok_or(E_INV_RESP)?);
                break;
            }
            let ligne = HrLigne::new(
                id,
                activity,
                worked_hours,
                product_name.unwrap_or("?".to_string()),
                product_description_sale.unwrap_or("?".to_string()),
                product_list_price.unwrap_or(0.0),
            );

            let fmt_date_odoo = FMT_DATE_ODOO;
            let fmt =
                chrono::format::strftime::StrftimeItems::new(FMT_DATE_INVOICE);

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
                vec_string.push("".to_string());
                for (ligne, note) in data.ligne_note.iter() {
                    vec_string.push(format!(
                        "{:<49} {:<10} {:<10} {:<10}",
                        format!("[{}] {}", ligne.activity, ligne.product_name),
                        format!("{:02}", ligne.worked_hours),
                        format!("{:02}", ligne.product_list_price),
                        format!(
                            "{:.2}",
                            ligne.worked_hours * ligne.product_list_price
                        ),
                    ));
                    vec_string
                        .push(format!("{}", ligne.product_description_sale));
                    vec_string.push(note.as_str().to_string());
                    vec_string.push("".to_string());
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
    pub product_name: String,
    pub product_description_sale: String,
    pub product_list_price: f64,
}

impl HrLigne {
    pub fn new(
        id: i32,
        activity: String,
        worked_hours: f64,
        product_name: String,
        product_description_sale: String,
        product_list_price: f64,
    ) -> Self {
        Self {
            id,
            activity,
            worked_hours,
            product_name,
            product_description_sale,
            product_list_price,
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
