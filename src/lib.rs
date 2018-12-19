extern crate cfg_if;
extern crate image;
extern crate js_sys;
extern crate wasm_bindgen;

mod edge;
mod utils;

use cfg_if::cfg_if;
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

// #[wasm_bindgen]
// extern {
//     fn alert(s: &str);
// }

#[wasm_bindgen]
pub fn detect(buf: Clamped<Vec<u8>>, width: u32, height: u32) -> Clamped<Vec<u8>> {
    let mut source_buf = image::RgbaImage::from_vec(width, height, buf.to_vec())
        .expect("Could not load image from buf");

    let image = image::DynamicImage::ImageRgba8(source_buf.clone()).to_luma();

    edge::canny(
        image,
        &mut source_buf,
        1.2,
        0.4,  // strong threshold
        0.05, // weak threshold
    );

    Clamped(source_buf.to_vec())
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
        let out = detect(raw, width, height);
        let out_buf = image::RgbaImage::from_vec(width, height, out.to_vec())
            .expect("Could not load image from buf");

        out_buf.save("test.jpeg").expect("Could not save test file");

        assert!(true);
    }
}
