//! https://www.odoo.com/documentation/14.0/webservices/odoo.html
extern crate xmlrpc;

mod error;
use error::Error;
use error::E_INV_CRED;
//use error::E_INV_RESP;
mod cfg;
use cfg::parse;
use xmlrpc::{Request, Value};

//use std::collections::BTreeMap;
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
    let request_common: String = format!("{}/xmlrpc/2/object", url.clone());
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
        .call_url(request_common.as_str())?;
    println!("{:?}", resp);
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
