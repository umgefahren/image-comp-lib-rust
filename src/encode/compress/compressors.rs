use bytes::{Bytes, Buf};
use std::io;
use crate::encode::flatten::lists::bytes_list;
use zstd::stream::Encoder;
use std::io::Write;


pub fn deflate(inp: &Bytes) -> Bytes {
    let mut data = inp.to_owned().reader();
    let mut encoder = libflate::deflate::Encoder::new(Vec::new());
    io::copy(&mut data, &mut encoder).unwrap();
    let encoded_data = encoder.finish().into_result().unwrap();
    bytes_list(&encoded_data)
}


pub fn comp_data(inp: &Bytes) -> Bytes {
    let data= inp.to_vec();
    let mut encoder = Encoder::new(Vec::new(), 21).unwrap();
    // encoder.multithread(n_counters).unwrap();
    encoder.write_all(&data).unwrap();
    let encoded_data = encoder.finish().unwrap();
    bytes_list(&encoded_data)
}
