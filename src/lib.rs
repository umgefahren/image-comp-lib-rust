#![allow(dead_code)]

/// Library of the Image Compression Algorithm. Explanation on how it works is available at GitHub:  [ReadMe](https://github.com/umgefahren/image-comp-lib-rust/blob/main/README.md)

#[macro_use]
pub mod io;
pub mod encode;
pub mod decode;
mod debug;

use std::path::PathBuf;
use bytes::Bytes;
use std::fs::File;
use std::io::{Write, Read};
use crate::io::load_image::load_image;
use crate::encode::encode::comp_img;
use crate::decode::decoder::con_img;

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use image::{RgbImage, Rgb, ImageResult};
    use crate::decode::lists::decode;
    use crate::debug::compare_img;
    use crate::encode::flatten::lists::{flatten_list, bytes_list};
    use crate::decode::construct::lists::{create_list, list_f_bytes};
    use crate::encode::compress::deflate::deflate;
    use crate::decode::compress::deflate::deflate_dec;
    use crate::encode::grid::grid_obj::from_list;
    use crate::encode::grid::grid_ops::{calc_cluster_map, calc_grid, calc_data_lists, calc_cluster_colors};
    use crate::encode::clustering::gen_point_cloud::{gen_point_cloud, gen_euclid_cloud};
    use crate::encode::clustering::clustering_methods::kmeans_clustering;
    use crate::encode::flatten::cluster_colors::flatten_cc;
    use crate::decode::construct::cluster_colors::create_cluster_colors;
    use crate::encode::encode::comp_img;
    use crate::decode::decoder::con_img;
    use std::fs;

    #[test]
    fn clustering_test_no_loss() {
        let p = PathBuf::from("./images/img_2.png");
        let img = crate::io::load_image::load_image(p);
        let pixel_count = img.img.dimensions().0 as usize * img.img.dimensions().1 as usize;
        let cloud = crate::encode::clustering::gen_point_cloud::gen_euclid_cloud(&img);
        let cluster = crate::encode::clustering::clustering_methods::kmeans_clustering(&cloud, 3);
        let mut size: usize = 0;
        for c in cluster.iter() {
            size += c.1.len();
        }
        assert_eq!(size, pixel_count);
    }

    #[test]
    fn clustering_test_debug() -> ImageResult<()>{
        let p = PathBuf::from("./images/img_2.png");
        let img = crate::io::load_image::load_image(p);
        let cloud = crate::encode::clustering::gen_point_cloud::gen_euclid_cloud(&img);
        let points = crate::encode::clustering::gen_point_cloud::gen_point_cloud(&img);
        let cluster = crate::encode::clustering::clustering_methods::kmeans_clustering(&cloud, 3);
        let mut debug_img = RgbImage::new(img.img.dimensions().0, img.img.dimensions().1);
        for (_idx, c) in cluster.iter().enumerate() {
            let mut add_color: [u32; 3] = [0, 0, 0];
            let mut counter: u32 = 1;
            for p in &c.1 {
                counter += 1;
                add_color[0] = add_color[0] + points[*p][2];
                add_color[1] = add_color[1] + points[*p][3];
                add_color[2] = add_color[2] + points[*p][4];
            }
            add_color[0] = add_color[0] / counter;
            add_color[1] = add_color[1] / counter;
            add_color[2] = add_color[2] / counter;
            let add_color = [add_color[0] as u8, add_color[1] as u8, add_color[2] as u8];
            let def_color: Rgb<u8> = Rgb(add_color);
            /*
            if idx == 0 {
                def_color = Rgb([255, 0, 0]);
            } else if idx == 1 {
                def_color = Rgb([0, 255, 0]);
            } else if idx == 2 {
                def_color = Rgb([0, 0, 255]);
            } else if idx == 3 {
                def_color = Rgb([255, 255, 0]);
            } else if idx == 4 {
                def_color = Rgb([255, 0, 255]);
            } else if idx == 5 {
                def_color = Rgb([0, 255, 255]);
            } else if idx == 6 {
                def_color = Rgb([255, 255, 255]);
            } else if idx == 7 {
                def_color = Rgb([0, 0, 0]);
            }
            */
            for p in &c.1 {
                debug_img.put_pixel(points[*p][0] as u32, points[*p][1] as u32, def_color);
            }
        }
        debug_img.save("./images/out.png")
    }

    #[test]
    fn cluster_map_test() {
        let p = PathBuf::from("./images/img_2.png");
        let img = crate::io::load_image::load_image(p);
        let cloud = crate::encode::clustering::gen_point_cloud::gen_euclid_cloud(&img);
        let cluster = crate::encode::clustering::clustering_methods::kmeans_clustering(&cloud, 3);
        let points = crate::encode::clustering::gen_point_cloud::gen_point_cloud(&img);
        let dims = img.dim();
        let ret = crate::encode::grid::grid_ops::calc_cluster_map(&cluster, &points, dims);
        let mut debug_img = RgbImage::new(img.img.dimensions().0, img.img.dimensions().1);
        for y in 0..dims[1] {
            for x in 0..dims[0] {
                let val = (ret[[x, y]] * 10) as u8;
                debug_img.put_pixel(x as u32, y as u32, Rgb([val, val, val]))
            }
        }
        debug_img.save("./images/out_cluster_map.png").unwrap();
        assert_eq!(dims, ret.shape());
    }

    #[test]
    fn grid_map_test_and_debug() {
        let p = PathBuf::from("./images/img_2.png");
        let img = crate::io::load_image::load_image(p);
        let cloud = crate::encode::clustering::gen_point_cloud::gen_euclid_cloud(&img);
        let cluster = crate::encode::clustering::clustering_methods::kmeans_clustering(&cloud, 3);
        let points = crate::encode::clustering::gen_point_cloud::gen_point_cloud(&img);
        let dims = &img.dim();
        let cluster_map = crate::encode::grid::grid_ops::calc_cluster_map(&cluster, &points, *dims);
        let grid = crate::encode::grid::grid_ops::calc_grid(&cluster_map, 10);
        let img_out_grid = grid.render();
        img_out_grid.save("./images/out_grid.png").unwrap();
        assert_eq!(*dims, grid.image_dim());
    }

    #[test]
    fn points_test_debug() {
        let p = PathBuf::from("./images/img_2.png");
        let img = crate::io::load_image::load_image(p);
        let points = crate::encode::clustering::gen_point_cloud::gen_point_cloud(&img);
        let mut debug_img = RgbImage::new(img.img.dimensions().0, img.img.dimensions().1);
        for p in points {
            debug_img.put_pixel(p[0], p[1], Rgb([p[2] as u8, p[3] as u8, p[4] as u8]));
        }
        debug_img.save("./images/out_points.png").unwrap();
    }

    #[test]
    fn lists_test() {
        let p = PathBuf::from("./images/img_2.png");
        let img = crate::io::load_image::load_image(p);
        let cloud = crate::encode::clustering::gen_point_cloud::gen_euclid_cloud(&img);
        let cluster = crate::encode::clustering::clustering_methods::kmeans_clustering(&cloud, 3);
        let points = crate::encode::clustering::gen_point_cloud::gen_point_cloud(&img);
        let dims = &img.dim();
        let cluster_map = crate::encode::grid::grid_ops::calc_cluster_map(&cluster, &points, *dims);
        let grid = crate::encode::grid::grid_ops::calc_grid(&cluster_map, 10);
        let cluster_colors = crate::encode::grid::grid_ops::calc_cluster_colors(&cluster, &points);
        let lists = crate::encode::grid::grid_ops::calc_data_lists(&img, &grid, &cluster_colors);
        let norm = &lists[0];
        let abs = &lists[1];
        let norm_f = flatten_list(norm);
        let abs_f = flatten_list(abs);
        let norm_r = create_list(norm_f);
        let abs_r = create_list(abs_f);
        let res_lists = [norm_r, abs_r];
        assert_eq!(lists, res_lists);
    }

    #[test]
    fn list_debug() {
        let p = PathBuf::from("./images/img_1.png");
        let img = crate::io::load_image::load_image(p);
        let cloud = crate::encode::clustering::gen_point_cloud::gen_euclid_cloud(&img);
        let cluster = crate::encode::clustering::clustering_methods::kmeans_clustering(&cloud, 3);
        let points = crate::encode::clustering::gen_point_cloud::gen_point_cloud(&img);
        let dims = &img.dim();
        let cluster_map = crate::encode::grid::grid_ops::calc_cluster_map(&cluster, &points, *dims);
        let grid = crate::encode::grid::grid_ops::calc_grid(&cluster_map, 100);
        let cluster_colors = crate::encode::grid::grid_ops::calc_cluster_colors(&cluster, &points);
        let lists = crate::encode::grid::grid_ops::calc_data_lists(&img, &grid, &cluster_colors);
        let norm = &lists[0];
        let abs = &lists[1];
        let norm_f = flatten_list(norm);
        let abs_f = flatten_list(abs);
        let norm_r = create_list(norm_f);
        let abs_r = create_list(abs_f);
        let res_lists = [norm_r, abs_r];
        let res_img = decode(&res_lists[0], &res_lists[1], &grid, &cluster_colors);
        res_img.save("./images/decoded_list.png").unwrap();
        compare_img(&img.img, &res_img);
        assert_eq!(img.img, res_img);
    }
    #[test]
    fn deflate_test() {
        let p = PathBuf::from("./images/img_2.png");
        let img = crate::io::load_image::load_image(p);
        let cloud = gen_euclid_cloud(&img);
        let cluster = kmeans_clustering(&cloud, 3);
        let points = gen_point_cloud(&img);
        let dims = &img.dim();
        let cluster_map = calc_cluster_map(&cluster, &points, *dims);
        let grid = calc_grid(&cluster_map, 100);
        let cluster_colors = calc_cluster_colors(&cluster, &points);
        let lists = calc_data_lists(&img, &grid, &cluster_colors);
        let norm = &lists[0];
        let abs = &lists[1];
        let norm_f = flatten_list(norm);
        let bytes_n = bytes_list(&norm_f);
        let comp_n = deflate(&bytes_n);
        let decomp_n = deflate_dec(&comp_n);
        let abs_f = flatten_list(abs);
        let bytes_a = bytes_list(&abs_f);
        let comp_a = deflate(&bytes_a);
        let decomp_a = deflate_dec(&comp_a);
        let decomp_n_l = list_f_bytes(&decomp_n);
        let decomp_a_l = list_f_bytes(&decomp_a);
        let norm_r = create_list(decomp_n_l);
        let abs_r = create_list(decomp_a_l);
        let res_lists = [norm_r, abs_r];
        println!("Uncompressed Size: {}", bytes_n.len() + bytes_a.len());
        println!("Compressed size:   {}", comp_n.len() + comp_a.len());
        let res_img = decode(&res_lists[0], &res_lists[1], &grid, &cluster_colors);
        res_img.save("./images/decoded_comp_list.png").unwrap();
        assert_eq!(lists, res_lists);
    }
    #[test]
    fn deflate_grid() {
        let p = PathBuf::from("./images/img_2.png");
        let img = crate::io::load_image::load_image(p);
        let cloud = gen_euclid_cloud(&img);
        let cluster = kmeans_clustering(&cloud, 3);
        let points = gen_point_cloud(&img);
        let dims = &img.dim();
        let cluster_map = calc_cluster_map(&cluster, &points, *dims);
        let grid = calc_grid(&cluster_map, 10);
        let g_list = grid.to_list();
        let g_b_list = bytes_list(&g_list);
        let comp_gb_list = deflate(&g_b_list);
        println!("Uncompressed size: {}", g_b_list.len());
        println!("Compressed size:   {}", comp_gb_list.len());
        let dec_comp_gb_list = deflate_dec(&comp_gb_list);
        let dec_g_list = list_f_bytes(&dec_comp_gb_list);
        let grid_n = from_list(&dec_g_list);
        assert_eq!(grid, grid_n);
    }

    #[test]
    fn flatten_cluster_colors() {
        let p = PathBuf::from("./images/img_1.png");
        let img = crate::io::load_image::load_image(p);
        let cloud = gen_euclid_cloud(&img);
        let cluster = kmeans_clustering(&cloud, 15);
        let points = gen_point_cloud(&img);
        let cluster_colors = calc_cluster_colors(&cluster, &points);
        let flat_cc = flatten_cc(&cluster_colors);
        let cc_b = bytes_list(&flat_cc);
        let comp_cc_b = deflate(&cc_b);
        println!("Uncompressed Size: {}", cc_b.len());
        println!("Compressed Size:   {}", comp_cc_b.len());
        let ret_cc = create_cluster_colors(&flat_cc);
        assert_eq!(cluster_colors, ret_cc);
    }

    #[test]
    fn encode_test() {
        let p = PathBuf::from("./images/img_4.png");
        let org_len = fs::metadata(&p).unwrap().len();
        let img = crate::io::load_image::load_image(p);
        let bs = comp_img(&img, 10, 3);
        println!("Compressed Img Size {} Bytes", bs.len());
        println!("Original Img Size   {} Bytes", org_len);
        let img2 = con_img(&bs);
        img2.save("./images/decompressed.png").unwrap();
        assert_eq!(img2, img.img)
    }
}

fn write_f(target_p: PathBuf, data: &Bytes) {
    let mut file = File::create(target_p).unwrap();
    file.write_all(data).unwrap();
}

fn read_f(target: PathBuf) -> Bytes {
    let mut buffer = Vec::new();
    let mut f = File::open(target).unwrap();
    f.read_to_end(&mut buffer).unwrap();
    Bytes::from(buffer)
}

/// Compress Image
/// 1. Input --> String describing the input path
/// 2. Input --> String describing the output path
pub fn compress_image(org_s: &String, target_s: &String) {
    let org_p = PathBuf::from(org_s);
    let target_p = PathBuf::from(target_s);
    let img = load_image(org_p);
    let bs = comp_img(&img, 10, 3);
    write_f(target_p, &bs);
}

/// Decompress Image
/// 1. Input --> String describing the input path
/// 2. Input --> String describing the output path
pub fn decompress_image(org_s: &String, target_s: &String) {
    let org_p = PathBuf::from(org_s);
    let target_p = PathBuf::from(target_s);
    let bs = read_f(org_p);
    let img = con_img(&bs);
    img.save(target_p).unwrap();
}