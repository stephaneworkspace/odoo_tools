use std::ffi::{CStr, CString};
use std::os::raw::c_char;

// Simple test pour essayer à nouveau SwiftUI
#[no_mangle]
pub extern "C" fn odoo_swift(hello: *const c_char) -> *const c_char {
    let hello_c_cstr = unsafe { CStr::from_ptr(hello) };
    let hello_string = hello_c_cstr.to_str().unwrap_or("Error UTF8 !");
    let result = format!("{} -> Odoo", hello_string);
    let result_cstring = CString::new(result).unwrap();
    result_cstring.as_ptr()
}
