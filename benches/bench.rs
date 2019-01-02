#![feature(test)]

extern crate edge_detection_wasm;
extern crate image;
extern crate js_sys;
extern crate test;
extern crate wasm_bindgen;

use image::GenericImageView;

#[bench]
fn detect_edge(b: &mut test::Bencher) {
    let img = image::open("test_images/building_small.jpg").unwrap();
    let width = img.width();
    let height = img.height();
    let raw = wasm_bindgen::Clamped(img.to_rgba().into_raw());

    b.iter(|| {
        edge_detection_wasm::detect(raw.clone(), width, height, 1);
    });
}
