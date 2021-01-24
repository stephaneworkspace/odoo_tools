//! https://www.odoo.com/documentation/14.0/webservices/odoo.html
extern crate serde_json;
extern crate xmlrpc;
#[macro_use]
extern crate simple_error;

use serde::Deserialize;
use serde::Serialize;
mod error;
use error::Error;
use error::E_INV_CRED;
//use error::E_INV_RESP;
mod cfg;
use cfg::parse;
use serde_json::value as json;
use std::collections::BTreeMap;
use xmlrpc::{Request, Value};

// try another one
extern crate xml_rpc;
mod rosrust;
use rosrust::client::Client as Client2;
//use rosrust::Response as Result;
use std::collections::HashMap;
use xml_rpc::{Client, Params, Url, Value as Value2};
mod api;
use api::master::Master;
use std::str::FromStr;
use url;

fn call_new_lib(url: Url, db: &str, uid: u32, password: &str, method: &str) {
    // ros
    let master: Master = Master::new(url.as_str(), "0", "1", db, uid, password, method).unwrap();

    // mix ros + xml_rpc
    let mut vec_field: Vec<&str> = Vec::new();
    vec_field.push("employee_id");
    master.set_param_fields(vec_field);
    let mut vec_value: Vec<Value2> = Vec::new();
    vec_value = master.get_param("fields").unwrap();
    // xml_rpc
    let mut client = Client::new().unwrap();
    let result = client
        .call_value(&url, "hr.attendance".to_string(), vec_value)
        .unwrap();
    println!("{:?}", result)
}

/// To simplify definitions using the XML-RPC "struct" type
//type OdooDataMap = BTreeMap<String, Value>;

fn main() -> Result<(), Error> {
    let clap = parse();
    let connection = clap.connection;
    let url: &str = &connection.url.as_str();
    let db: &str = &connection.db.as_str();
    let username: &str = &connection.username.as_str();
    let password: &str = &connection.password.as_str();
    let invoice_date_in = clap.invoice_date_in;
    let invoice_date_out = clap.invoice_date_out;
    let uid: i32 = login(&url, &db, &username, &password)?;
    println!(
        "{:?}",
        hr_id_query(
            &url,
            &db,
            uid.clone(),
            &password,
            invoice_date_in,
            invoice_date_out
        )
    );
    Ok(())
    //let request_start: String = format!("https://{}/start", url.clone());
    //let request_common: String = format!("https://{}/xmlrpc/2/common", url.clone());

    /*    let request = Request::new(request_start.as_str())i
          .arg(Value::Struct(
            vec![
                ("host".to_string(), Value::from(url.clone())),
                ("db".to_string(), Value::from(db.clone())),
                ("username".to_string(), Value::from(username.clone())),
                ("password".to_string(), Value::from(password.clone())),
            ]
            .into_iter()
            .collect(),
        ));
        println!("{:?}", request.call_url(request_common.as_str()));
    */
}

fn login(url: &str, db: &str, username: &str, password: &str) -> Result<i32, Error> {
    let request_common: String = format!("{}/xmlrpc/2/common", url.clone());
    let resp = Request::new("authenticate")
        .arg(db)
        .arg(username)
        .arg(password)
        .arg(Value::Nil)
        /*.arg(Value::Struct(
            vec![
                ("db".to_string(), Value::from(db.clone())),
                ("username".to_string(), Value::from(username.clone())),
                ("password".to_string(), Value::from(password.clone())),
                ("".to_string(), Value::Struct(BTreeMap::new())),
            ]
            .into_iter()
            .collect(),
        ))*/
        .call_url(request_common.as_str())?;
    match resp.as_i32() {
        Some(value) => Ok(value),
        None => Err(E_INV_CRED),
    }
    //println!("{:?}", resp);
    /*
        val_to_response_simple(&resp)?
            .get("authenticate")
            .and_then(Value::as_i32)
            .map(i32::from)
            //.map(String::from)
            .ok_or(E_INV_RESP)
    */
}

#[derive(Serialize, Deserialize)]
struct JsonReqValue {
    fields: Vec<String>,
}

fn hr_id_query(
    url: &str,
    db: &str,
    uid: i32,
    password: &str,
    invoice_date_in: String,
    invoice_date_out: String,
) -> Result<(), Error> {
    let date_in = iso8601::datetime(&invoice_date_in.to_string()).unwrap();
    let date_out = iso8601::datetime(&invoice_date_out.to_string()).unwrap();
    let request_object: String = format!("{}/xmlrpc/2/object", url.clone());
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
    let resp = Request::new("execute_kw")
        .arg(db.clone())
        .arg(uid.clone())
        .arg(password.clone())
        .arg("hr.attendance")
        .arg("search")
        .arg(Value::Array(vec_read1))
        /*.arg(Value::Struct(
            vec![
                ("db".to_string(), Value::from(db.clone())),
                ("username".to_string(), Value::from(username.clone())),
                ("password".to_string(), Value::from(password.clone())),
                ("".to_string(), Value::Struct(BTreeMap::new())),
            ]
            .into_iter()
            .collect(),
        ))*/
        .call_url(request_object.as_str())?;

    // Read key
    let mut vec_select: Vec<Value> = Vec::new();
    vec_select.push(Value::String("employee_id".to_string()));
    //vec_select.push(Value::from("write_uid"));
    let mut btree = BTreeMap::new();
    btree.insert(
        "fields".to_string(),
        Value::Array(vec_select.into_iter().collect()),
    );
    //btree.insert("fields".to_string(), Value::from("employee_id"));
    //btree.insert("limit".to_string(), Value::from(5));
    let btree_value = Value::Struct(btree);

    let json_value = r#"{
        "fields": [
            "employee_id"
        ]
    }"#;
    let mut vec_select2: Vec<String> = Vec::new();
    vec_select2.push("employee_id".to_string());
    let json_value2: JsonReqValue = JsonReqValue {
        fields: vec_select2.to_vec(),
    };
    let json_value3: JsonReqValue = serde_json::from_str(json_value).unwrap();
    let serialize = serde_json::to_string(&json_value3).unwrap();
    let serialize_str: &str = serialize.as_str();

    call_new_lib(
        url::Url::from_str(&*format!("{}/xmlrpc/2/object", url.clone())).unwrap(),
        db,
        uid as u32,
        password,
        "read",
    );
    println!("{:?}", resp);
    /*
    let read = Request::new("execute_kw")
        .arg(db.clone())
        .arg(uid.clone())
        .arg(password.clone())
        .arg("hr.attendance")
        .arg("read")
        .arg(resp.clone())
        /*.arg(Value::Struct(
            vec![("fields".to_string(), Value::Array(vec_select))]
                .into_iter()
                .collect(),
        ))*/
        .arg(Value::from_json(&json::Value::from(serialize_str)))
        //.arg(btree_value)
        //.arg(Value::Array(vec![btree_value]))
        .call_url(request_object.as_str())?;

    println!("{:?} {:?}", resp, read);
    */
    Ok(())
    /*val_to_response_btree(&resp)?
    .get("id")
    .and_then(Value::as_str)
    .map(String::from)
    .ok_or(E_INV_RESP)*/
}
/*
fn val_to_response_simple(v: &Value) -> Result<&Value, Error> {
    let resp = v.as_struct().ok_or(E_INV_RESP)?;

    let status = resp
        .get("status")
        .and_then(Value::as_str)
        .ok_or(E_INV_RESP)?;

    if status.starts_with("200") {
        Ok(v)
    } else {
        Err(Error::Ost(
            format!("xmlrpc request failed: {}", status).into(),
        ))
    }
}*/
/*
fn val_to_response_btree(v: &Value) -> Result<&OdooDataMap, Error> {
    let resp = v.as_struct().ok_or(E_INV_RESP)?;

    let status = resp
        .get("status")
        .and_then(Value::as_str)
        .ok_or(E_INV_RESP)?;

    if status.starts_with("200") {
        Ok(resp)
    } else {
        Err(Error::Ost(
            format!("xmlrpc request failed: {}", status).into(),
        ))
    }
}*/

/// Utilities to extract RPC response value and convert them to serde json values
pub trait RpcHelpers {
    /// Translate the json value into an equivalent xmlrpc value.
    /// This is not 1-1, as we have no reasonable way to detect
    /// base64, date or i32 values from json.
    fn from_json(value: &json::Value) -> xmlrpc::Value;
    /// Convert the RPC value to a serde json value
    fn as_json(&self) -> json::Value;
    /* Get the "Value" field from a RPC response
    fn rpc_value(&self) -> XapiResult<&xmlrpc::Value>;
     */
}

impl RpcHelpers for xmlrpc::Value {
    fn from_json(value: &json::Value) -> xmlrpc::Value {
        match *value {
            json::Value::Number(ref i) if i.is_i64() => {
                let n = i.as_i64().unwrap();
                xmlrpc::Value::Int64(n)
            }
            // not an i64, we make it into f64
            json::Value::Number(ref i) => {
                let n = i.as_f64().unwrap();
                xmlrpc::Value::Double(n)
            }
            json::Value::Bool(b) => xmlrpc::Value::Bool(b),
            json::Value::String(ref s) => xmlrpc::Value::String(s.clone()),
            json::Value::Object(ref jmap) => {
                let mut map = BTreeMap::new();
                for (ref name, ref v) in jmap {
                    map.insert(name.to_string().clone(), Self::from_json(v));
                }
                xmlrpc::Value::Struct(map)
            }
            json::Value::Array(ref array) => {
                xmlrpc::Value::Array(array.iter().map(|v| Self::from_json(v)).collect())
            }
            json::Value::Null => xmlrpc::Value::Nil,
        }
    }

    fn as_json(&self) -> json::Value {
        match *self {
            xmlrpc::Value::Int(i) => {
                let i = json::Number::from_f64(i as f64).unwrap();
                json::Value::Number(i)
            }
            xmlrpc::Value::Int64(i) => {
                let i = json::Number::from_f64(i as f64).unwrap();
                json::Value::Number(i)
            }
            xmlrpc::Value::Bool(b) => json::Value::Bool(b),
            xmlrpc::Value::String(ref s) => json::Value::String(s.clone()),
            xmlrpc::Value::Double(d) => {
                let d = json::Number::from_f64(d).unwrap();
                json::Value::Number(d)
            }
            // TODO remove this after solving the 2 next lines
            _ => json::Value::Null,
            // TODO xmlrpc::Value::DateTime(date_time) => json::Value::String(format_datetime(&date_time)),
            // TODO xmlrpc::Value::Base64(ref data) => json::Value::String(encode(data)),
            xmlrpc::Value::Struct(ref map) => {
                let mut jmap = serde_json::Map::with_capacity(map.len());
                for (ref name, ref v) in map {
                    jmap.insert(name.to_string().clone(), v.as_json());
                }
                json::Value::Object(jmap)
            }
            xmlrpc::Value::Array(ref array) => {
                json::Value::Array(array.iter().map(|v| v.as_json()).collect())
            }
            xmlrpc::Value::Nil => json::Value::Null,
        }
    }
    /*
    fn rpc_value(&self) -> XapiResult<&xmlrpc::Value> {
        match *self {
            xmlrpc::Value::Struct(ref response) if response.contains_key("Value") => {
                Ok(&response["Value"])
            }
            xmlrpc::Value::Struct(ref response) if response.contains_key("ErrorDescription") => {
                bail!(format!(
                    "XML Rpc error: {}",
                    serde_json::to_string(&response["ErrorDescription"].as_json())?
                ))
            }
            _ => bail!(format!("Unkown error: {:?}", self)),
        }
    }*/
}
