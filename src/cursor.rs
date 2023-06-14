use crate::{Result, Error};

pub struct Cursor<T> {
    inner: T,
    position: usize,
}

impl<T: AsRef<[u8]>> Cursor<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            position: 0,
        }
    }

    pub fn position(&self) -> usize {
        self.position
    }

    pub fn read_exact(&mut self, buf: &mut [u8]) -> Result<()> {
        let inner = self.inner.as_ref();
        let len = buf.len();
        let remaining = inner.len() - self.position;

        if len > remaining {
            return Err(Error::InsufficientLength);
        }

        let start = self.position;
        let end = start + len;

        debug_assert!(end - start == len);

        let slice = &inner[start..end];

        buf.copy_from_slice(slice);

        self.position = end;

        Ok(())
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

        assert_eq!(Err(Error::InsufficientLength), cursor.read_exact(&mut buf));
    }
}
