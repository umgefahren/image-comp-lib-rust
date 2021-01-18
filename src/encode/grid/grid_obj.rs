use ndarray::{Axis, Array2, Array};
use image::{RgbImage, Rgb};

#[derive(Clone, PartialEq, Debug)]
pub struct Grid {
    pub w: usize,
    pub h: usize,
    pub wx: usize,
    pub hx: usize,
    pub data: Array2<u8>,
}

impl Grid {
    pub fn render(&self) -> RgbImage {
        let mut img = RgbImage::new(self.wx as u32, self.hx as u32);
        for (y, item) in self.data.axis_iter(Axis(1)).enumerate() {
            for (x, pixel) in item.axis_iter(Axis(0)).enumerate() {
                let val = pixel.iter().next().unwrap() * 10;
                let mut color = Rgb([val, val, val]);
                let pix = *pixel.iter().next().unwrap();
                if pix == 16 {
                    color = Rgb([255, 255, 255]);
                } else if pix == 0 {
                    color = Rgb([255, 0, 0]);
                } else if pix == 1 {
                    color = Rgb([0, 255, 0]);
                } else if pix == 2 {
                    color = Rgb([0, 0, 255]);
                }
                img.put_pixel(x as u32, y as u32, color);
            }
        }
        img
    }
    pub fn image_dim(self) -> [usize; 2] {
        [self.data.dim().0 * self.w, self.data.dim().1 * self.h]
    }
    pub fn to_list(&self) -> Vec<u8> {
        let mut ret = vec![];
        let w = self.w as u64;
        let mut w = w.to_be_bytes().iter().map(|s| s.to_owned()).collect();
        ret.append(&mut w);
        let h = self.h as u64;
        let mut h = h.to_be_bytes().iter().map(|s| s.to_owned()).collect();
        ret.append(&mut h);
        let wx = self.wx as u64;
        let mut wx = wx.to_be_bytes().iter().map(|s| s.to_owned()).collect();
        ret.append(&mut wx);
        let hx = self.hx as u64;
        let mut hx = hx.to_be_bytes().iter().map(|s| s.to_owned()).collect();
        ret.append(&mut hx);
        let mut lines: Vec<u8> = self.data.iter().map(|f| f.to_owned()).collect();
        // let lines: Vec<u8> = lines.iter().map(|f| f.to_owned().to_owned()).collect();
        // println!("Lines: {:?}", lines);
        ret.append(&mut lines);
        ret
    }
}

pub fn from_list(inp: &[u8]) -> Grid {
    let mut w_b: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
    w_b[..8].clone_from_slice(&inp[..8]);
    let w = u64::from_be_bytes(w_b) as usize;
    let mut h_b: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
    h_b[..8].clone_from_slice(&inp[8..(8 + 8)]);
    let h = u64::from_be_bytes(h_b) as usize;
    let mut wx_b: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
    wx_b[..8].clone_from_slice(&inp[16..(8 + 16)]);
    let wx = u64::from_be_bytes(wx_b) as usize;
    let mut hx_b: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
    hx_b[..8].clone_from_slice(&inp[24..(8 + 24)]);
    let hx = u64::from_be_bytes(hx_b) as usize;

    let mut arr: Array2<u8> = Array::zeros((wx, hx));
    let mut lines_iter = inp[32..].iter();
    for wn in 0..wx {
        for hn in 0..hx {
            let obj = lines_iter.next().unwrap().to_owned();
            arr[[wn, hn]] = obj;
        }
    }
    Grid {
        w,
        h,
        wx,
        hx,
        data: arr,
    }
}