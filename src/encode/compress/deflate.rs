use bytes::{Bytes, Buf};

use std::io;
use crate::encode::flatten::lists::bytes_list;

pub fn deflate(inp: &Bytes) -> Bytes {
    let mut data = inp.to_owned().reader();
    let mut encoder = libflate::deflate::Encoder::new(Vec::new());
    io::copy(&mut data, &mut encoder).unwrap();
    let encoded_data = encoder.finish().into_result().unwrap();
    bytes_list(&encoded_data)
}