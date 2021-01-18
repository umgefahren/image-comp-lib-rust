use image::open;
use nshare::ToNdarray3;
use std::path::PathBuf;
use crate::io::img_obj::Image;
use ndarray::Array3;

pub fn load_image(p: PathBuf) -> Image {
    let img = open(p).unwrap().into_rgb8();
    let data: Array3<u8> = img.clone().to_ndarray3();
    Image {
        img,
        data,
    }
}