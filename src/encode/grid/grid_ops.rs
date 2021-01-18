use cogset::Euclid;
use ndarray::{Array, ArrayBase, OwnedRepr, Dim, Axis, Array2, ArrayView, Ix3, s};
use crate::encode::grid::grid_obj::Grid;
use crate::io::img_obj::Image;
use image::{Rgb, RgbImage};
use std::collections::HashMap;
use crate::encode::clustering::cluster_color::calc_min;

pub fn calc_cluster_map(cluster: &[(Euclid<[f64; 5]>, Vec<usize>)], points: &[[u32; 5]], shape: [usize; 2]) -> ArrayBase<OwnedRepr<usize>, Dim<[usize; 2]>> {
    let mut arr = Array::zeros(shape);
    for (id, c) in cluster.iter().enumerate() {
        for idx in &c.1 {
            let x = points[*idx][0] as usize;
            let y = points[*idx][1] as usize;
            arr[[x, y]] = id;
        }
    }
    arr
}

pub fn calc_grid(cluster_map: &ArrayBase<OwnedRepr<usize>, Dim<[usize; 2]>>, min_size: usize) -> Grid {
    let x_len = cluster_map.dim().0;
    let mut min_x: usize = 1;
    if min_size < x_len {
        for c in min_size..(x_len - 2) {
            if x_len % c == 0 {
                min_x = c;
                break;
            }
        }
    }
    if min_x == 1 {
        let mut iter = min_size..2;
        if min_size > x_len {
            iter = 2..x_len;
        }
        for c in iter {
        if x_len % c == 0 {
            min_x = c;
            break;
        }
    }
    }
    let y_len = cluster_map.dim().1;
    let mut min_y: usize = 1;
    if min_size < y_len {
        for c in min_size..(y_len - 2) {
            if y_len % c == 0 {
                min_y = c;
                break;
            }
        }
    }
    if min_y == 1 {
        let mut iter = min_size..2;
        if min_size > y_len {
            iter = 2..y_len;
        }
        for c in iter {
            if y_len % c == 0 {
                min_y = c;
                break;
            }
        }
    }
    let mut arr: Array2<u8> = Array::zeros((x_len / min_x, y_len / min_y));
    let x_arr = cluster_map.axis_chunks_iter(Axis(0), min_x);
    for (idx, x) in x_arr.enumerate() {
        let y_arr = x.axis_chunks_iter(Axis(1), min_y);
        for (idy, y) in y_arr.enumerate() {
            let tmp = y.iter().next().unwrap();
            let mut consistent = true;
            for e in y.iter() {
                if *e != *tmp {
                    consistent = false;
                    break;
                }
            }
            if consistent {
                arr[[idx, idy]] = tmp.to_owned() as u8;
            } else {
                // println!("ref: {} check: {}", tmp, different);
                arr[[idx, idy]] = 16;
            }
        }
    }
    Grid {
        w: min_x,
        h: min_y,
        wx: x_len / min_x,
        hx: y_len / min_y,
        data: arr,
    }
}

fn calc_chunck_abs(chunck: &ArrayView<u8, Ix3>) -> Vec<[u8; 3]> {
    // println!("{:?}", chunck);
    let mut pixels = vec![];
    for idx in 0..chunck.dim().2 {
        let y_iter: Vec<usize> = if idx % 2 == 0 {
            (0..chunck.dim().1).collect()
        } else {
            (0..chunck.dim().1).rev().collect()
        };
        // let y_iter: Vec<usize> = (0..chunck.dim().1).collect();
        for idy in y_iter {
            let pixel = [chunck[[0, idy, idx]], chunck[[1, idy, idx]], chunck[[2, idy, idx]]];
            pixels.push(pixel);
        }
    }
    pixels
}

fn calc_chunck_rel(chunck: &ArrayView<u8, Ix3>, color: &[u8; 3]) -> Vec<[u8; 3]> {
    let c_r = color[0];
    let c_g = color[1];
    let c_b = color[2];
    let mut pixels = vec![];
    for idx in 0..chunck.dim().2 {
        let y_iter: Vec<usize> = if idx % 2 == 0 {
            (0..chunck.dim().1).collect()
        } else {
            (0..chunck.dim().1).rev().collect()
        };
        // let y_iter: Vec<usize> = (0..chunck.dim().1).collect();
        for idy in y_iter {
            let pixel = [chunck[[0, idy, idx]], chunck[[1, idy, idx]], chunck[[2, idy, idx]]];
            pixels.push(pixel);
        }
    }
    pixels.iter().map(|p| { [p[0] - c_r, p[1] - c_g, p[2] - c_b] }).collect()
}

pub fn calc_cluster_colors(clusters: &[(Euclid<[f64; 5]>, Vec<usize>)], points: &[[u32; 5]]) -> HashMap<u8, [u8; 3]> {
    let mut cluster_colors = HashMap::new();
    for (idx, cluster) in clusters.iter().enumerate() {
        cluster_colors.insert(idx as u8, calc_min(&cluster.1, points));
    }
    cluster_colors
}

pub fn calc_data_lists(img: &Image, grid: &Grid, cluster_colors: &HashMap<u8, [u8; 3]>) -> [Vec<[u8; 3]>; 2] {
    let mut debug_img = RgbImage::new(img.dim()[0] as u32, img.dim()[1] as u32);
    let mut norm_chunks = vec![];
    let mut abs_chunks = vec![];
    for idy in 0..grid.hx {
        for idx in 0..grid.wx {
            let x_b = idx * grid.w;
            let mut x_e = (idx + 1) * grid.w;
            let y_b = idy * grid.h;
            let mut y_e = (idy + 1) * grid.h;
            // println!("Dimensions: {:?}", img.data.dim());
            // println!("X_b: {} X_e: {}", x_b, x_e);
            // println!("Y_b: {} Y_e: {}", y_b, y_e);
            // let test = img.data[[0, y_b, x_b]];
            // println!("1: {}", test);
            // let test = img.data[[0, y_e, x_e]];
            // println!("2: {}", test);
            // println!("--->>");
            for y in y_b..y_e {
                for x in x_b..x_e {
                    debug_img.put_pixel(x as u32, y as u32, Rgb([255, 255, 255]));
                }
            }
            if x_e >= img.data.dim().2 - 1 {
                x_e = img.data.dim().2;
            }
            if y_e >= img.data.dim().1 - 1 {
                y_e = img.data.dim().1;
            }
            let chunck = img.data.slice(s![.., y_b..y_e, x_b..x_e]);
            // println!("Chunck Size ({}, {})", grid.w, grid.h);
            // println!("Chunck Dims: {:?}", chunck.dim());
            // println!("Super Counter: {}", super_counter);
            let val = grid.data[[idx, idy]];
            if val == 16 {
                let mut res = calc_chunck_abs(&chunck);
                abs_chunks.append(&mut res);
            } else {
                let mut res = calc_chunck_rel(&chunck, cluster_colors.get(&val).unwrap());
                norm_chunks.append(&mut res);
            }
        }
    }
    // println!("Internal counter: {}", counter);
    // debug_img.save("./images/debug_img.png").unwrap();
    [norm_chunks, abs_chunks]
}