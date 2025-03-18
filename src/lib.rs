pub mod entrypoint;
pub mod registrator;
pub mod variant;
pub mod xlauto;
pub mod xlcall;

extern crate widestring;
extern crate winapi;

pub use registrator::Reg;
use thiserror::Error;

#[derive(Error, Debug)]
#[error("XLAddError: {msg}")]
pub struct XLAddError {
    msg: String,
}

impl XLAddError {
    pub fn invalid_data(msg: &str) -> Self {
        Self {
            msg: msg.to_string(),
        }
    }
}
