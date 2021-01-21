//! https://www.odoo.com/documentation/14.0/webservices/odoo.html
extern crate xmlrpc;

mod cfg;
use cfg::parse;
use xmlrpc::{Request, Value};

fn main() {
    let clap = parse();
    let connection = clap.connection;
    let url: &str = &connection.url.as_str();
    let db: &str = &connection.db.as_str();
    let username: &str = &connection.username.as_str();
    let password: &str = &connection.password.as_str();

    let request = Request::new(&connection.url).arg(Value::Struct(
        vec![
            ("host".to_string(), Value::from(url.clone())),
            ("db".to_string(), Value::from(db.clone())),
            ("username".to_string(), Value::from(username.clone())),
            ("password".to_string(), Value::from(password.clone())),
        ]
        .into_iter()
        .collect(),
    ));
    println!("{:?}", request);
}
