pub mod hr;
mod invoice;
mod master;

pub use self::hr::{Hr, HrData, HrJson};
pub use self::master::OdooConnection;
pub use self::invoice::{Invoice, InvoiceData};
