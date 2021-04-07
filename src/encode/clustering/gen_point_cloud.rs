use crate::io::img_obj::Image;
use ndarray::Axis;

pub fn gen_euclid_cloud(img: &Image) -> Vec<[f64; 5]> {
    let arr = &img.data;
    let x_len = img.data.dim().2 as f64;
    let y_len = img.data.dim().1 as f64;
    let mut super_vec: Vec<[f64; 5]> = vec![];
    for (y, item) in arr.axis_iter(Axis(1)).enumerate() {
        for (x, pixel) in item.axis_iter(Axis(1)).enumerate() {
            // super_vec.push(Euclid([pixel[0] as f64, pixel[1] as f64, pixel[2] as f64]));
            super_vec.push([pixel[0] as f64, pixel[1] as f64, pixel[2] as f64, (x as f64 / x_len) * 255.0, (y as f64 / y_len) * 255.0]);
        }
    }
    super_vec
}


pub fn gen_point_cloud(img: &Image) -> Vec<[u32; 5]> {
    let arr = &img.data;
    let mut super_vec: Vec<[u32; 5]> = vec![];
    for (y, item) in arr.axis_iter(Axis(1)).enumerate() {
        for (x, pixel) in item.axis_iter(Axis(1)).enumerate() {
            super_vec.push([x as u32, y as u32, pixel[0] as u32, pixel[1] as u32, pixel[2] as u32]);
        }
    }
    super_vec
}