extern crate cfg_if;
extern crate image;
extern crate js_sys;
extern crate wasm_bindgen;

mod edge;
mod utils;

use cfg_if::cfg_if;
use image::Pixel;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
pub fn detect(buf: Clamped<Vec<u8>>, width: u32, height: u32, count: u32) -> Clamped<Vec<u8>> {
    let buf_vec = buf.0;

    let mut source_buffer = image::RgbaImage::from_vec(width, height, buf_vec)
        .expect("Could not load image from input buffer");

    let mut gray_image = image::DynamicImage::new_luma8(width, height).to_luma();

    for (x, y, p) in source_buffer.enumerate_pixels() {
        gray_image.put_pixel(x, y, p.to_luma());
    }

    edge::canny(&gray_image, &mut source_buffer, 150.0, 300.0, count);

    Clamped(source_buffer.into_raw())
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::*;

    #[test]
    fn detect_works() {
        let img = image::open("test_images/building_small.jpg").unwrap();
        let width = img.width();
        let height = img.height();
        let raw = Clamped(img.to_rgba().into_raw());
        let out = detect(raw, width, height, 1);
        let out_buf = image::RgbaImage::from_vec(width, height, out.to_vec())
            .expect("Could not load image from buf");

        out_buf.save("test.jpeg").expect("Could not save test file");

        assert!(true);
    }
}
