extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate xmlrpc;
mod error;
mod odoo_const;
use error::Error;
mod api;
use api::OdooConnection;
use api::{Hr, HrData, HrJson};
mod cfg;
use cfg::{Connection, HrSelection};
use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime};
use std::ffi::{CStr, CString, c_void};
use std::os::raw::c_char;
use std::os::raw::c_int;
use std::os::raw::c_uint;
use std::thread;
use std::time::Duration;

/// Simple test pour essayer à nouveau SwiftUI
#[no_mangle]
pub extern "C" fn odoo_swift(hello: *const c_char) -> *mut c_char {
    let hello_c_cstr = unsafe { CStr::from_ptr(hello) };
    let hello_string = hello_c_cstr.to_str().unwrap_or("Error UTF8 !");
    let result = format!("{} -> Odoo", hello_string);
    let result_cstring = CString::new(result).unwrap();
    result_cstring.into_raw()
}

/// Get day work
#[no_mangle]
pub extern "C" fn get_work(
    url: *const c_char,
    db: *const c_char,
    username: *const c_char,
    password: *const c_char,
    year: c_int,
    month: c_uint,
    day: c_uint,
) -> *mut c_char {
    let url_c_cstr = unsafe { CStr::from_ptr(url) };
    let url_str = url_c_cstr.to_str().unwrap_or("Error UTF8 !");
    let db_c_cstr = unsafe { CStr::from_ptr(db) };
    let db_str = db_c_cstr.to_str().unwrap_or("Error UTF8 !");
    let username_c_cstr = unsafe { CStr::from_ptr(username) };
    let username_str = username_c_cstr.to_str().unwrap_or("Error UTF8 !");
    let password_c_cstr = unsafe { CStr::from_ptr(password) };
    let password_str = password_c_cstr.to_str().unwrap_or("Error UTF8 !");
    let connection_struct = Connection {
        url: url_str.to_string(),
        db: db_str.to_string(),
        username: username_str.to_string(),
        password: password_str.to_string(),
    };
    let mut connection = OdooConnection::new(connection_struct);
    connection.login().unwrap(); // TODO

    let date_in = NaiveDate::from_ymd(year, month, day).and_hms(0, 0, 0);
    let date_out =
        chrono::NaiveDate::from_ymd(year, month, day).and_hms(23, 59, 59);

    let invoice_date = format!("{:4}-{:02}-{:02}", year, month, day);

    let date_in1: DateTime<FixedOffset> = DateTime::from_utc(
        NaiveDateTime::from_timestamp(date_in.timestamp(), 0),
        FixedOffset::east(0),
    );
    let date_out1: DateTime<FixedOffset> = DateTime::from_utc(
        NaiveDateTime::from_timestamp(date_out.timestamp(), 0),
        FixedOffset::east(0),
    );

    let hr_selection: HrSelection = HrSelection {
        invoice_date,
        invoice_date_in: date_in1.to_rfc3339(),
        invoice_date_out: date_out1.to_rfc3339(),
    };

    let mut hr = HrData::new(connection, hr_selection);
    hr.selection().unwrap(); // TODO
    let result_cstring = CString::new(hr.data_to_json()).unwrap();
    result_cstring.into_raw()
}

#[no_mangle]
pub extern "C" fn free_string(s: *mut c_char) {
    let cstring = unsafe { CString::from_raw(s) };
    drop(cstring); // not technically required but shows what we're doing
}

/// Try to implement: https://www.nickwilcox.com/blog/recipe_swift_rust_callback/
#[no_mangle]
pub extern "C" fn async_operation(callback: CompletedCallback) {
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(3));
        callback.succeeded()
    });
}

#[repr(C)]
pub struct CompletedCallback {
    userdata: *mut c_void,
    callback: extern "C" fn(*mut c_void, bool),
}

unsafe impl Send for CompletedCallback {}

impl CompletedCallback {
    pub fn succeeded(self) {
        (self.callback)(self.userdata, true);
        std::mem::forget(self)
    }
    pub fn failed(self) {
        (self.callback)(self.userdata, false);
        std::mem::forget(self)
    }
}

impl Drop for CompletedCallback {
    fn drop(&mut self) {
        panic!("CompletedCallback must have explicit succeeded or failed call")
    }
}