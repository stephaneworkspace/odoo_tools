#[derive(Debug)]
pub enum Arrondi {
    _Arrondi5Centime = 1, // Arrondi à 5 centimes
}

impl<'s> Arrondi {
    pub fn _table(self) -> &'s str {
        "account_cach_rounding"
    }
}

pub const PRODUCT_PRODUCT_ID_UNKNOWN: i32 = 18;
pub const FMT_DATE_ODOO: &str = "%Y-%m-%d %H:%M:%S";
pub const FMT_DATE_INVOICE: &str = "%H:%M";
