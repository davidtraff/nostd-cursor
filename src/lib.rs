#![no_std]

pub mod cursor;

#[cfg(feature = "byteorder")]
mod bo;

#[cfg(feature = "byteorder")]
pub use bo::*;

#[derive(Debug, PartialEq)]
pub enum Error {
    UnexpectedEof,
}

pub type Result<T> = core::result::Result<T, Error>;