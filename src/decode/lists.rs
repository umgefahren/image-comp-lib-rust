use crate::encode::grid::grid_obj::Grid;
use image::{RgbImage, Rgb};
use std::collections::HashMap;


pub fn decode(norm_list: &[[u8; 3]], abs_list: &[[u8; 3]], grid: &Grid, cluster_colors: &HashMap<u8, [u8; 3]>) -> RgbImage {
    let mut res_img = RgbImage::new((grid.w * grid.wx) as u32, (grid.h * grid.hx) as u32);
    let w_len = grid.w * grid.wx + 1;
    let h_len = grid.h * grid.hx + 1;
    let mut abs_iter = abs_list.iter();
    let mut norm_iter = norm_list.iter();
    for idy in 0..grid.hx {
        // println!("{:?}", x_arr);
        for idx in 0..grid.wx {
            let mut real_w = grid.w;
            let mut real_h = grid.h;
            if ((idx + 1) * grid.w) >= w_len {
                real_w -= 1;
            }
            if ((idy + 1) * grid.h) >= h_len {
                real_h -= 1;
            }
            let mut chunck_pixel_value: Vec<[u8; 3]> = vec![];
            let code = grid.data[[idx, idy]];
            if code == 16 {
                let mut add_list = vec![];
                for _n in 0..(real_w * real_h) {
                    let pixel = match abs_iter.next() {
                        Some(d) => d.to_owned(),
                        None => {
                            println!("Missing pixel!");
                            [255, 255, 255]
                        },
                    };
                    add_list.push(pixel);
                }
                chunck_pixel_value.append(&mut add_list);
                /*if chunck_pixel_value.len() != 6 {
                    println!("{} chunk len", chunck_pixel_value.len());
                }*/
            } else {
                let base_color = cluster_colors.get(&code).unwrap().to_owned();
                let mut add_list = vec![];
                for _n in 0..(real_w * real_h) {
                    let pixel = match norm_iter.next() {
                        Some(d) => [d[0].to_owned() as i16, d[1].to_owned() as i16, d[2].to_owned() as i16],
                        None => {
                            println!("Missing Pixel!");
                            [base_color[0] as i16 * -1, base_color[1] as i16 * -1, base_color[2] as i16 * -1]
                        },
                    };
                    let res_pixel = [(pixel[0] + base_color[0] as i16) as u8, (pixel[1] + base_color[1] as i16) as u8, (pixel[2] + base_color[2] as i16) as u8];
                    add_list.push(res_pixel);
                }
                chunck_pixel_value.append(&mut add_list);
                /*if chunck_pixel_value.len() != 6 {
                    println!("{} chunk len", chunck_pixel_value.len());
                }*/
            }
            let mut chunck_iter = chunck_pixel_value.iter();
            let mut counter = 0;
            // println!("Idxe: {} Idye: {}", (idx + 1) * grid.w, (idy + 1) * grid.h);
            for x in (idx * grid.w)..((idx + 1) * grid.w) {
                if x >= w_len - 1 {
                    continue
                }
                let y_iter: Vec<usize> = if counter % 2 == 0 {
                    ((idy * grid.h)..(idy + 1) * grid.h).collect()
                } else {
                    ((idy * grid.h)..(idy + 1) * grid.h).rev().collect()
                };
                counter += 1;
                // let y_iter: Vec<usize> = ((idy * grid.h)..(idy + 1) * grid.h).collect();
                for y in y_iter {
                    // println!("x: {} idx: {} y: {} idy: {} {:?}", x, idx, y, idy, res_img.dimensions());
                    if y >= h_len - 1 {
                        continue
                    }
                    let color = match chunck_iter.next() {
                        Some(d) => Rgb(d.to_owned()),
                        None => {
                            println!("Missing pixel Value");
                            Rgb([0, 0, 0])
                        }
                    };
                    res_img.put_pixel(x as u32, y as u32, color);
                }
            }
        }
    }
    // println!("res_img dim {:?}", res_img.dimensions());
    res_img
}