#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

extern crate image;
extern crate js_sys;
extern crate wasm_bindgen;

mod edge;
mod utils;

use image::{GrayImage, Pixel, RgbaImage};
use std::sync::Mutex;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;

static mut INITIALIZED: bool = false;

lazy_static! {
  // Since it's mutable and shared, use mutext.
  static ref GRAY_IMAGE: Mutex<image::GrayImage> = Mutex::new(GrayImage::new(640, 480));
}

#[wasm_bindgen]
pub fn detect(
    buf: Clamped<Vec<u8>>,
    width: u32,
    height: u32,
    hue: u32,
    use_thick: bool,
) -> Clamped<Vec<u8>> {
    unsafe {
        if !INITIALIZED {
            edge::initialize(width, height);
        }
    };
    let buf_vec = buf.0;

    // create image from image buffer
    let mut source_buffer = RgbaImage::from_vec(width, height, buf_vec)
        .expect("Could not load image from input buffer");

    // convert image buffer to grayscale (luma) buffer;
    let mut gray_image = GRAY_IMAGE.lock().unwrap();
    for (x, y, p) in source_buffer.enumerate_pixels() {
        gray_image.put_pixel(x, y, p.to_luma());
    }

    // create gray image from gray image buffer
    edge::canny(
        &gray_image,
        &mut source_buffer,
        150.0,
        300.0,
        hue,
        use_thick,
    );

    // clamp results for canvas
    Clamped(source_buffer.into_raw())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::edge::atan2_approx;
    use image::*;
    use test::Bencher;

    const ORANGE: u32 = 4288554239;

    #[test]
    fn detect_works() {
        let img = image::open("test_images/test.jpg").unwrap();
        let width = img.width();
        let height = img.height();
        let raw = Clamped(img.to_rgba().into_raw());
        let out = detect(raw, width, height, ORANGE);
        let out_buf = image::RgbaImage::from_vec(width, height, out.to_vec())
            .expect("Could not load image from buf");

        out_buf.save("test.jpg").expect("Could not save test file");

        assert!(true); // TODO: write real tests
    }

    #[bench]
    fn bench_atan2(b: &mut Bencher) {
        b.iter(|| (230.0 as f32).atan2(200.0));
    }

    #[bench]
    fn bench_atan2_approx(b: &mut Bencher) {
        b.iter(|| atan2_approx(230.0, 200.0));
    }
}
