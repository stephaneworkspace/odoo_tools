#[derive(Debug)]
pub enum Arrondi {
    _Arrondi5Centime = 1, // Arrondi à 5 centimes
}

impl<'s> Arrondi {
    pub fn _table(self) -> &'s str {
        "account_cach_rounding"
    }
}
