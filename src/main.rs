extern crate xmlrpc;
mod error;
mod odoo_const;
use error::Error;
mod cfg;
use cfg::parse;
mod api;
use api::OdooConnection;
use api::{Hr, HrData};

fn main() -> Result<(), Error> {
    let clap = parse();
    let mut connection = OdooConnection::new(clap.connection);
    connection.login()?;
    let mut hr = HrData::new(connection, clap.hr_selection);
    hr.selection()?;
    Ok(())
}
