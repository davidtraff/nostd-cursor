use crate::{Result, Error};

pub struct Cursor<T> {
    inner: T,
    position: usize,
}

impl<T> Cursor<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            position: 0,
        }
    }

    pub fn position(&self) -> usize {
        self.position
    }
}

impl<T: AsRef<[u8]>> Cursor<T> {
    pub fn remaining_slice(&self) -> &[u8] {
        let len = self.position.min(self.inner.as_ref().len());

        &self.inner.as_ref()[len..]
    }

    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let slice = self.remaining_slice();
        let amt = buf.len().min(slice.len());
        let a = &slice[..amt];

        if amt == 1 {
            buf[0] = a[0];
        } else {
            buf[..amt].copy_from_slice(a);
        }
        
        Ok(amt)
    }

    pub fn read_exact(&mut self, buf: &mut [u8]) -> Result<()> {
        let slice = self.remaining_slice();
        if buf.len() > slice.len() {
            return Err(Error::UnexpectedEof);
        }
        let a = &slice[..buf.len()];

        if buf.len() == 1 {
            buf[0] = a[0];
        } else {
            buf.copy_from_slice(a);
        }

        self.position += buf.len();

        Ok(())
    }
}

impl<T: AsMut<[u8]>> Cursor<T> {
    pub fn remaining_slice_mut(&mut self) -> &mut [u8] {
        let len = self.position.min(self.inner.as_mut().len());

        &mut self.inner.as_mut()[len..]
    }

    pub fn write(&mut self, data: &[u8]) -> Result<usize> {
        let slice = self.remaining_slice_mut();
        let amt = data.len().min(slice.len());

        slice.copy_from_slice(&data[..amt]);

        self.position += amt;
        
        Ok(amt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_exact_increments_position() {
        const LEN: usize = 3;

        let mut cursor = Cursor::new([0u8, 1, 2, 3, 4]);
        let mut buf = [0u8; LEN];

        cursor.read_exact(&mut buf).unwrap();

        assert_eq!(LEN, cursor.position());
    }

    #[test]
    fn read_exact_reads_correctly() {
        const LEN: usize = 3;

        let mut cursor = Cursor::new([0u8, 1, 2, 3, 4]);
        let mut buf = [0u8; LEN];

        cursor.read_exact(&mut buf).unwrap();

        assert_eq!([0, 1, 2], buf);
    }

    #[test]
    fn read_exact_gives_error_on_invalid_len() {
        const LEN: usize = 8;

        let mut cursor = Cursor::new([0u8, 1, 2, 3, 4]);
        let mut buf = [0u8; LEN];

        assert_eq!(Err(Error::UnexpectedEof), cursor.read_exact(&mut buf));
    }
}
