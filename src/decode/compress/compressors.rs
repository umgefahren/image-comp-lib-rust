use bytes::{Bytes, Buf};
use libflate::deflate::Decoder;
use std::io::Read;
use zstd::decode_all;

pub fn deflate_dec(inp: &Bytes) -> Bytes {
    let data = inp.to_owned().reader();
    let mut decoder = Decoder::new(data);
    let mut decoded_data = Vec::new();
    decoder.read_to_end(&mut decoded_data).unwrap();
    Bytes::from(decoded_data)
}


pub fn dec_comp_data(inp: &Bytes) -> Bytes {
    let data = inp.to_owned().reader();
    // let mut decoder = Decoder::new(data);
    let decoded_data = decode_all(data).unwrap();
    // decoder.read_to_end(&mut decoded_data).unwrap();
    Bytes::from(decoded_data)
}