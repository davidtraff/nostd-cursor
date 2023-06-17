use crate::cursor::Cursor;

#[test]
fn test_mem_writer() {
    let mut writer = Cursor::new([0u8; 256]);
    writer.set_position(10);
    assert_eq!(writer.write(&[0]).unwrap(), 1);
    assert_eq!(writer.write(&[1, 2, 3]).unwrap(), 3);
    assert_eq!(writer.write(&[4, 5, 6, 7]).unwrap(), 4);
    let b: &[_] = &[0, 1, 2, 3, 4, 5, 6, 7];
    assert_eq!(&writer.get_ref()[..10], &[0; 10]);
    assert_eq!(&writer.get_ref()[10..(10 + b.len())], b);
}

#[test]
fn test_mem_writer_preallocated() {
    let mut writer = Cursor::new([0u8, 0, 0, 0, 0, 0, 0, 0, 8, 9, 10]);
    assert_eq!(writer.write(&[0]).unwrap(), 1);
    assert_eq!(writer.write(&[1, 2, 3]).unwrap(), 3);
    assert_eq!(writer.write(&[4, 5, 6, 7]).unwrap(), 4);
    let b: &[_] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    assert_eq!(&writer.get_ref()[..], b);
}

#[test]
fn test_array_writer() {
    let mut writer = Cursor::new([0u8; 9]);
    
    assert_eq!(writer.position(), 0);
    assert_eq!(writer.write(&[0]).unwrap(), 1);
    assert_eq!(writer.position(), 1);
    assert_eq!(writer.write(&[1, 2, 3]).unwrap(), 3);
    assert_eq!(writer.write(&[4, 5, 6, 7]).unwrap(), 4);
    assert_eq!(writer.position(), 8);
    assert_eq!(writer.write(&[]).unwrap(), 0);
    assert_eq!(writer.position(), 8);

    assert_eq!(writer.write(&[8, 9]).unwrap(), 1);
    assert_eq!(writer.write(&[10]).unwrap(), 0);
    let b: &[_] = &[0, 1, 2, 3, 4, 5, 6, 7, 8];
    assert_eq!(writer.get_ref().as_ref(), b);
}

#[test]
fn test_buf_writer_error() {
    let mut buf = [0 as u8; 2];
    let mut writer = Cursor::new(&mut buf[..]);
    assert_eq!(writer.write(&[0]).unwrap(), 1);
    assert_eq!(writer.write(&[0, 0]).unwrap(), 1);
    assert_eq!(writer.write(&[0, 0]).unwrap(), 0);
}

#[test]
fn test_mem_reader() {
    let mut reader = Cursor::new([0, 1, 2, 3, 4, 5, 6, 7]);
    let mut buf = [];
    assert_eq!(reader.read(&mut buf).unwrap(), 0);
    assert_eq!(reader.position(), 0);
    let mut buf = [0];
    assert_eq!(reader.read(&mut buf).unwrap(), 1);
    assert_eq!(reader.position(), 1);
    let b: &[_] = &[0];
    assert_eq!(buf, b);
    let mut buf = [0; 4];
    assert_eq!(reader.read(&mut buf).unwrap(), 4);
    assert_eq!(reader.position(), 5);
    let b: &[_] = &[1, 2, 3, 4];
    assert_eq!(buf, b);
    assert_eq!(reader.read(&mut buf).unwrap(), 3);
    let b: &[_] = &[5, 6, 7];
    assert_eq!(&buf[..3], b);
    assert_eq!(reader.read(&mut buf).unwrap(), 0);
}

#[test]
fn test_buf_reader() {
    let in_buf = [0, 1, 2, 3, 4, 5, 6, 7];
    let mut reader = Cursor::new(&in_buf[..]);
    let mut buf = [];
    assert_eq!(reader.read(&mut buf).unwrap(), 0);
    assert_eq!(reader.position(), 0);
    let mut buf = [0];
    assert_eq!(reader.read(&mut buf).unwrap(), 1);
    assert_eq!(reader.position(), 1);
    let b: &[_] = &[0];
    assert_eq!(buf, b);
    let mut buf = [0; 4];
    assert_eq!(reader.read(&mut buf).unwrap(), 4);
    assert_eq!(reader.position(), 5);
    let b: &[_] = &[1, 2, 3, 4];
    assert_eq!(buf, b);
    assert_eq!(reader.read(&mut buf).unwrap(), 3);
    let b: &[_] = &[5, 6, 7];
    assert_eq!(&buf[..3], b);
    assert_eq!(reader.read(&mut buf).unwrap(), 0);
}