use bytes::{BytesMut, BufMut};
use std::io;
use crate::Message;
use simd_json_derive::Serialize;

pub struct Writer<'a>(pub &'a mut BytesMut);

impl<'a> io::Write for Writer<'a> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.put_slice(buf);
        Ok(buf.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub fn to_body(message: Message) -> BytesMut {
    let mut body = BytesMut::with_capacity(10);
    message.json_write(&mut Writer(&mut body)).unwrap();
    body
}