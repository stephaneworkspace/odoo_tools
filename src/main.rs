//! https://www.odoo.com/documentation/14.0/webservices/odoo.html

extern crate xmlrpc;

mod cfg;
use cfg::parse;
use xmlrpc::{Request, Value};

use std::collections::BTreeMap;
/// To simplify definitions using the XML-RPC "struct" type
type OstDataMap = BTreeMap<String, Value>;

// errors TODO in another file
use std::borrow::Cow;
use std::borrow::Cow::Borrowed;
use std::io;
use xmlrpc::{Error as RequestError, Fault};

/// A commonly used Error
pub(crate) const E_INV_CRED: Error = Error::Ost(Borrowed("invalid credential"));
pub(crate) const E_INV_RESP: Error = Error::Ost(Borrowed("invalid xml-rpc response"));
/// All the errors that can occur
#[derive(Debug)]
pub(crate) enum Error {
    Io(io::Error),
    Ost(Cow<'static, str>),
    XmlRpcRequest(RequestError),
    XmlRpcFault(Fault),
    Reqwest(reqwest::Error),
}

/// Converting all sub-errors into Error.

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::Io(e)
    }
}

impl From<&'static str> for Error {
    fn from(e: &'static str) -> Error {
        Error::Ost(e.into())
    }
}

impl From<RequestError> for Error {
    fn from(e: RequestError) -> Error {
        Error::XmlRpcRequest(e)
    }
}

impl From<Fault> for Error {
    fn from(e: Fault) -> Error {
        Error::XmlRpcFault(e)
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Error {
        Error::Reqwest(e)
    }
}

/// Prints an error to stderr
pub(crate) fn print_err(err: String) {
    eprintln!("{}", err);
}

/// If the input is an Error then prints it to stderr
pub(crate) fn print_if_err<T>(res: &Result<T, Error>) {
    if let Err(ref err) = res {
        match err {
            Error::Ost(ref e) => eprintln!("{}", e.to_string()),
            Error::Io(ref e) => eprintln!("{}", e.to_string()),
            Error::XmlRpcRequest(ref e) => eprintln!("{}", e.to_string()),
            Error::XmlRpcFault(ref e) => eprintln!("{}", e.to_string()),
            Error::Reqwest(ref e) => eprintln!("{}", e.to_string()),
        }
    }
}

fn main() {
    let clap = parse();
    let connection = clap.connection;
    let url: &str = &connection.url.as_str();
    let db: &str = &connection.db.as_str();
    let username: &str = &connection.username.as_str();
    let password: &str = &connection.password.as_str();
    println!("{:?}", login(url, db, username, password));

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
        .arg(db.clone())
        .arg(username.clone())
        .arg(password.clone())
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

fn val_to_response_btree(v: &Value) -> Result<&OstDataMap, Error> {
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
}
