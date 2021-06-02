use image_comp::io::load_image::load_image;
use std::path::PathBuf;
use std::str::FromStr;
use image_comp::encode::encode::comp_img;
use image_comp::decode::decoder::con_img;

use criterion::{criterion_group, criterion_main, Criterion};

fn comp_decomp() {
    let image = load_image(PathBuf::from_str("images/img_4.png").unwrap());
    let comped = comp_img(&image, 10, 3);
    let decomped = con_img(&comped);
    assert_eq!(image.img, decomped)
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("img_4");
    group.sample_size(10);
    group.bench_function("Comp Img", |b| b.iter(|| comp_decomp()));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);