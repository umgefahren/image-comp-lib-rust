use image::{RgbImage, Rgb};

pub fn compare_img(img1: &RgbImage, img2: &RgbImage) {
    let mut debug_img = RgbImage::new(img1.dimensions().0, img1.dimensions().1);
    let x_max = img1.dimensions().0;
    let y_max = img1.dimensions().1;
    for x in 0..x_max {
        for y in 0..y_max {
            let val1 = img1.get_pixel(x, y);
            let val2 = img2.get_pixel(x, y);
            let mut dif = [
                (val1[0] as isize - val2[0] as isize).abs() as u8 + 127,
                (val1[1] as isize - val2[1] as isize).abs() as u8 + 127,
                (val1[2] as isize - val2[2] as isize).abs() as u8 + 127
            ];
            if dif != [127, 127, 127] {
                dif = [255, 0, 0];
            }
            let col = Rgb(dif);
            debug_img.put_pixel(x, y, col);
        }
    }
    debug_img.save("./images/dif_image.png").unwrap();
}