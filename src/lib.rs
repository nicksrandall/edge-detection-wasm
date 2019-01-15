#![feature(test)]
extern crate test;

extern crate image;
extern crate js_sys;
extern crate wasm_bindgen;

mod edge;
mod utils;

use image::{GenericImageView, GrayImage, Pixel, RgbaImage};
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;

#[wasm_bindgen]
pub fn detect(
    buf: Clamped<Vec<u8>>,
    width: u32,
    height: u32,
    hue: u32,
    use_thick: bool,
) -> Clamped<Vec<u8>> {
    let buf_vec = buf.0;

    // create image from image buffer
    let mut source_buffer = RgbaImage::from_vec(width, height, buf_vec)
        .expect("Could not load image from input buffer");

    // convert image buffer to grayscale (luma) buffer;
    let gray_image = GrayImage::from_fn(width, height, |x, y| {
        (unsafe { source_buffer.unsafe_get_pixel(x, y).to_luma() })
    });

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
