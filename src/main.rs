use std::env;

use image_comp::{compress_image, decompress_image};

fn main() {
    let args: Vec<String> = env::args().collect();
    let org = &args[2];
    let target = &args[3];
    let op = &args[1].to_owned();
    if op == &String::from("dec") {
        compress_image(org, target);
    } else {
        decompress_image(org, target);
    }
}