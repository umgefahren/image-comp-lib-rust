use bytes::Bytes;
use image::RgbImage;
use crate::decode::compress::deflate::deflate_dec;
use crate::decode::construct::lists::{list_f_bytes, create_list};
use crate::encode::grid::grid_obj::from_list;
use crate::decode::construct::cluster_colors::create_cluster_colors;
use crate::decode::lists::decode;

pub fn con_img(inp: &Bytes) -> RgbImage {
    let mut b_iter = inp.iter();
    let mut parts = vec![];
    for x in 0..4 {
        let mut len_b = [0, 0, 0, 0, 0, 0, 0, 0];
        for n in 0..8  {
            len_b[n] = b_iter.next().unwrap().to_owned();
        }
        let len = u64::from_be_bytes(len_b);
        let mut buf = vec![];
        for _n in 0..len {
            buf.push(b_iter.next().unwrap().to_owned());
        }
        let mut b = Bytes::from(buf);
        if x != 1 {
            b = deflate_dec(&b);
        }
        parts.push(b);
    }
    let gl_c = &parts[0];
    let cc_b = &parts[1];
    let norm_c = &parts[2];
    let abs_c = &parts[3];
    let gl = list_f_bytes(gl_c);
    let cc_l = list_f_bytes(cc_b);
    let norm_l = list_f_bytes(norm_c);
    let abs_l = list_f_bytes(abs_c);
    let grid = from_list(&gl);
    let cc = create_cluster_colors(&cc_l);
    let norm = create_list(norm_l);
    let abs = create_list(abs_l);
    decode(&norm, &abs, &grid, &cc)
}