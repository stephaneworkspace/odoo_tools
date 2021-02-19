extern crate xmlrpc;
mod error;
mod odoo_const;
use error::Error;
mod cfg;
use cfg::parse;
mod api;
use api::OdooConnection;
use api::{Hr, HrData, Invoice, InvoiceData};
use api::hr::HrJson;

fn main() -> Result<(), Error> {
    let clap = parse();
    let mut connection = OdooConnection::new(clap.connection);
    connection.login()?;
    let mut hr = HrData::new(connection, clap.hr_selection);
    hr.selection()?;
    println!("{}", hr.data_total_to_json());
    /*
    println!("Invoice sent:");
    let mut invoice = InvoiceData::new(connection, "posted".to_string());
    invoice.selection()?;*/
    Ok(())
}
/*
fn main2() -> Result<(), Error> {
    let clap = parse();
    let mut connection = OdooConnection::new(clap.connection);
    connection.login()?;
    let mut hr = HrData::new(connection, clap.hr_selection);
    hr.selection()?;
    println!("{}", hr.data_to_str());
    Ok(())
}
*/