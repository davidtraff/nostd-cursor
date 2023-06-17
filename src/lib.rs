#![no_std]

pub mod cursor;

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