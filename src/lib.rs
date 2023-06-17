#![no_std]

mod cursor;

pub use cursor::Cursor;

#[cfg(feature = "byteorder")]
mod bo;

#[cfg(feature = "byteorder")]
pub use bo::*;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
pub enum Error {
    UnexpectedEof,
    InvalidInput,
}

pub type Result<T> = core::result::Result<T, Error>;

pub fn test() {
    
}