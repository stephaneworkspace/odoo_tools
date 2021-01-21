//! https://www.odoo.com/documentation/14.0/webservices/odoo.html

use clap::{App, Arg};

#[derive(Debug)]
pub struct Odoo {
    pub connection: Connection,
}

#[derive(Debug)]
pub struct Connection {
    pub url: String,
    pub db: String,
    pub username: String,
    pub password: String,
}

const URL: &str = "url";
const DB: &str = "db";
const USERNAME: &str = "username";
const PASSWORD: &str = "password";

pub fn parse() -> Odoo {
    let matches = App::new("Odoo stephane-bressani.ch cli tools")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Stéphane Bressani")
        .about("Tool for some operation")
        .arg(
            Arg::with_name(URL)
                .long("url")
                .value_name("URL")
                .help("Server URL")
                .required(true),
        )
        .arg(
            Arg::with_name(DB)
                .long("db")
                .value_name("DB")
                .help("Database name")
                .required(true),
        )
        .arg(
            Arg::with_name(USERNAME)
                .long("username")
                .value_name("USERNAME")
                .help("Database username")
                .required(true),
        )
        .arg(
            Arg::with_name(PASSWORD)
                .long("password")
                .value_name("PASSWORD")
                .help("Database password")
                .required(true),
        )
        .get_matches();
    Odoo {
        connection: Connection {
            url: matches.value_of(URL).unwrap().to_string(),
            db: matches.value_of(DB).unwrap().to_string(),
            username: matches.value_of(USERNAME).unwrap().to_string(),
            password: matches.value_of(PASSWORD).unwrap().to_string(),
        },
    }
}
