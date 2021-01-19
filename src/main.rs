use std::env;

use image_comp::{compress_image, decompress_image};

fn main() {
    let error_message = "\
Help message of the Image Compression Algorithm\n\
(C) Hannes Furmans -- see LICENSE for Licensing Information\n\
Have fun using it, although it's just experimental 🦭\n\
Arguments:\n\
First Argument:  comp (for compression) or dec (for decompression)\n\
Second Argument: Path to the image for compression, Path to the binary for decompression (Source Directory)\n\
Third Argument:  Path to the binary for compression, Path to the image for decompression (Target Directory)\n\
Example: image-compressor (may var) img.png img.bin\n\
Notify me if you are facing issues\n\
";
    let args: Vec<String> = env::args().collect();
    let org = args.get(2).expect(error_message);
    let target = args.get(3).expect(error_message);
    let op = args.get(1).expect(error_message);
    if op == &String::from("comp") {
        compress_image(org, target);
    } else if op == &String::from("dec") {
        decompress_image(org, target);
    } else if op == &String::from("-h") || op == &String::from("--help") {
        print!("{}", error_message);
    }
}