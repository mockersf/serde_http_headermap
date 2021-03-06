extern crate http;
extern crate serde;

mod de;
mod ser;

pub mod error;

pub use de::from_headermap;
pub use ser::to_headermap;
pub use error::Error;
