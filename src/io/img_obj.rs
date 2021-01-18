use image::RgbImage;
use ndarray::Array3;


/// Image Object
/// Rgb Image
/// Array From the RGB Image
#[derive(Clone)]
pub struct Image {
    pub img: RgbImage,
    pub data: Array3<u8>,
}

impl Image {
    pub fn dim(&self) -> [usize; 2] {
        [self.img.dimensions().0 as usize, self.img.dimensions().1 as usize]
    }
}