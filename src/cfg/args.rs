//! https://www.odoo.com/documentation/14.0/webservices/odoo.html
//use super::validator::validator_date_invoice;
use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime};
use clap::{App, Arg};

#[derive(Debug)]
pub struct Odoo {
    pub connection: Connection,
    pub hr_selection: HrSelection,
}

#[derive(Debug)]
pub struct Connection {
    pub url: String,
    pub db: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug)]
pub struct HrSelection {
    pub invoice_date: String,
    pub invoice_date_in: String,
    pub invoice_date_out: String,
}

const URL: &str = "url";
const DB: &str = "db";
const USERNAME: &str = "username";
const PASSWORD: &str = "password";
const INVOICE_DATE_AAAA: &str = "invoice_date_aaaa";
const INVOICE_DATE_MM: &str = "invoice_date_mm";
const INVOICE_DATE_DD: &str = "invoice_date_dd";

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
        .arg(
            Arg::with_name(INVOICE_DATE_AAAA)
                .long("aaaa")
                .value_name("INVOICE_DATE_AAAA")
                .help("Invoice date by hours in odoo aaaa")
                .required(true),
        )
        .arg(
            Arg::with_name(INVOICE_DATE_MM)
                .long("mm")
                .value_name("INVOICE_DATE_MM")
                .help("Invoice date by hours in odoo mm")
                .required(true),
        )
        .arg(
            Arg::with_name(INVOICE_DATE_DD)
                .long("dd")
                .value_name("INVOICE_DATE_DD")
                .help("Invoice date by hours in odoo dd")
                .required(true),
        )
        .get_matches();

    let date_in = NaiveDate::from_ymd(
        matches
            .value_of(INVOICE_DATE_AAAA)
            .unwrap()
            .to_string()
            .parse::<i32>()
            .unwrap(),
        matches
            .value_of(INVOICE_DATE_MM)
            .unwrap()
            .to_string()
            .parse::<u32>()
            .unwrap(),
        matches
            .value_of(INVOICE_DATE_DD)
            .unwrap()
            .to_string()
            .parse::<u32>()
            .unwrap(),
    )
    .and_hms(0, 0, 0);
    let date_out = chrono::NaiveDate::from_ymd(
        matches
            .value_of(INVOICE_DATE_AAAA)
            .unwrap()
            .to_string()
            .parse::<i32>()
            .unwrap(),
        matches
            .value_of(INVOICE_DATE_MM)
            .unwrap()
            .to_string()
            .parse::<u32>()
            .unwrap(),
        matches
            .value_of(INVOICE_DATE_DD)
            .unwrap()
            .to_string()
            .parse::<u32>()
            .unwrap(),
    )
    .and_hms(23, 59, 59);

    let invoice_date = format!(
        "{:4}-{:02}-{:02}",
        matches
            .value_of(INVOICE_DATE_AAAA)
            .unwrap()
            .to_string()
            .parse::<u32>()
            .unwrap(),
        matches
            .value_of(INVOICE_DATE_MM)
            .unwrap()
            .to_string()
            .parse::<u32>()
            .unwrap(),
        matches
            .value_of(INVOICE_DATE_DD)
            .unwrap()
            .to_string()
            .parse::<u32>()
            .unwrap()
    );
    let date_in1: DateTime<FixedOffset> = DateTime::from_utc(
        NaiveDateTime::from_timestamp(date_in.timestamp(), 0),
        FixedOffset::east(0),
    );
    let date_out1: DateTime<FixedOffset> = DateTime::from_utc(
        NaiveDateTime::from_timestamp(date_out.timestamp(), 0),
        FixedOffset::east(0),
    );

    Odoo {
        connection: Connection {
            url: matches.value_of(URL).unwrap().to_string(),
            db: matches.value_of(DB).unwrap().to_string(),
            username: matches.value_of(USERNAME).unwrap().to_string(),
            password: matches.value_of(PASSWORD).unwrap().to_string(),
        },
        hr_selection: HrSelection {
            invoice_date,
            invoice_date_in: date_in1.to_rfc3339(),
            invoice_date_out: date_out1.to_rfc3339(),
        },
    }
}
