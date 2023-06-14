#![no_std]

pub mod cursor;

use core::marker::PhantomData;

pub use byteorder::*;

#[derive(Debug, PartialEq)]
pub enum Error {
    InsufficientLength,
}

pub type Result<T> = core::result::Result<T, Error>;
pub type LECursor<T> = EndianCursor<LittleEndian, T>;
pub type BECursor<T> = EndianCursor<BigEndian, T>;

pub struct EndianCursor<B: ByteOrder, T: AsRef<[u8]>> {
    cursor: cursor::Cursor<T>,
    _phantom: PhantomData<B>,
}

impl<B: ByteOrder, T: AsRef<[u8]>> EndianCursor<B, T> {
    pub fn new(inner: T) -> Self {
        Self {
            cursor: cursor::Cursor::new(inner),
            _phantom: PhantomData,
        }
    }
    
    #[inline]
    pub fn read_u8(&mut self) -> Result<u8> {
        const LEN: usize = core::mem::size_of::<u8>();
        let mut buf = [0; LEN];
        
        self.cursor.read_exact(&mut buf)?;

        Ok(buf[0])
    }

    #[inline]
    pub fn read_i8(&mut self) -> Result<i8> {
        const LEN: usize = core::mem::size_of::<i8>();
        let mut buf = [0; LEN];
        
        self.cursor.read_exact(&mut buf)?;

        Ok(buf[0] as i8)
    }

    #[inline]
    pub fn read_u16(&mut self) -> Result<u16> {
        const LEN: usize = core::mem::size_of::<u16>();
        let mut buf = [0; LEN];
        
        self.cursor.read_exact(&mut buf)?;

        Ok(B::read_u16(&buf))
    }

    #[inline]
    pub fn read_i16(&mut self) -> Result<i16> {
        const LEN: usize = core::mem::size_of::<i16>();
        let mut buf = [0; LEN];
        
        self.cursor.read_exact(&mut buf)?;

        Ok(B::read_i16(&buf))
    }

    #[inline]
    pub fn read_u24(&mut self) -> Result<u32> {
        const LEN: usize = core::mem::size_of::<u32>();
        let mut buf = [0; LEN];
        
        self.cursor.read_exact(&mut buf)?;

        Ok(B::read_u24(&buf))
    }

    #[inline]
    pub fn read_i24(&mut self) -> Result<i32> {
        const LEN: usize = core::mem::size_of::<i32>();
        let mut buf = [0; LEN];
        
        self.cursor.read_exact(&mut buf)?;

        Ok(B::read_i24(&buf))
    }

    #[inline]
    pub fn read_u32(&mut self) -> Result<u32> {
        const LEN: usize = core::mem::size_of::<u32>();
        let mut buf = [0; LEN];
        
        self.cursor.read_exact(&mut buf)?;

        Ok(B::read_u32(&buf))
    }

    #[inline]
    pub fn read_i32(&mut self) -> Result<i32> {
        const LEN: usize = core::mem::size_of::<i32>();
        let mut buf = [0; LEN];
        
        self.cursor.read_exact(&mut buf)?;

        Ok(B::read_i32(&buf))
    }

    #[inline]
    pub fn read_u48(&mut self) -> Result<u64> {
        const LEN: usize = core::mem::size_of::<u64>();
        let mut buf = [0; LEN];
        
        self.cursor.read_exact(&mut buf)?;

        Ok(B::read_u48(&buf))
    }

    #[inline]
    pub fn read_i48(&mut self) -> Result<i64> {
        const LEN: usize = core::mem::size_of::<i64>();
        let mut buf = [0; LEN];
        
        self.cursor.read_exact(&mut buf)?;

        Ok(B::read_i48(&buf))
    }

    #[inline]
    pub fn read_u64(&mut self) -> Result<u64> {
        const LEN: usize = core::mem::size_of::<u64>();
        let mut buf = [0; LEN];
        
        self.cursor.read_exact(&mut buf)?;

        Ok(B::read_u64(&buf))
    }

    #[inline]
    pub fn read_i64(&mut self) -> Result<i64> {
        const LEN: usize = core::mem::size_of::<i64>();
        let mut buf = [0; LEN];
        
        self.cursor.read_exact(&mut buf)?;

        Ok(B::read_i64(&buf))
    }

    #[inline]
    pub fn read_u128(&mut self) -> Result<u128> {
        const LEN: usize = core::mem::size_of::<u128>();
        let mut buf = [0; LEN];
        
        self.cursor.read_exact(&mut buf)?;

        Ok(B::read_u128(&buf))
    }

    #[inline]
    pub fn read_i128(&mut self) -> Result<i128> {
        const LEN: usize = core::mem::size_of::<i128>();
        let mut buf = [0; LEN];
        
        self.cursor.read_exact(&mut buf)?;

        Ok(B::read_i128(&buf))
    }

    #[inline]
    pub fn read_uint(&mut self, nbytes: usize) -> Result<u64> {
        const LEN: usize = core::mem::size_of::<u64>();
        let mut buf = [0; LEN];
        
        self.cursor.read_exact(&mut buf[..nbytes])?;

        Ok(B::read_uint(&buf[..nbytes], nbytes))
    }

    #[inline]
    pub fn read_int(&mut self, nbytes: usize) -> Result<i64> {
        const LEN: usize = core::mem::size_of::<i64>();
        let mut buf = [0; LEN];
        
        self.cursor.read_exact(&mut buf[..nbytes])?;

        Ok(B::read_int(&buf[..nbytes], nbytes))
    }

    #[inline]
    pub fn read_uint128(&mut self, nbytes: usize) -> Result<u128> {
        const LEN: usize = core::mem::size_of::<u128>();
        let mut buf = [0; LEN];
        
        self.cursor.read_exact(&mut buf[..nbytes])?;

        Ok(B::read_uint128(&buf[..nbytes], nbytes))
    }

    #[inline]
    pub fn read_int128(&mut self, nbytes: usize) -> Result<i128> {
        const LEN: usize = core::mem::size_of::<i128>();
        let mut buf = [0; LEN];
        
        self.cursor.read_exact(&mut buf[..nbytes])?;

        Ok(B::read_int128(&buf[..nbytes], nbytes))
    }

    #[inline]
    pub fn read_f32(&mut self) -> Result<f32> {
        const LEN: usize = core::mem::size_of::<f32>();
        let mut buf = [0; LEN];
        
        self.cursor.read_exact(&mut buf)?;

        Ok(B::read_f32(&buf))
    }

    #[inline]
    pub fn read_f64(&mut self) -> Result<f64> {
        const LEN: usize = core::mem::size_of::<f64>();
        let mut buf = [0; LEN];
        
        self.cursor.read_exact(&mut buf)?;

        Ok(B::read_f64(&buf))
    }
}