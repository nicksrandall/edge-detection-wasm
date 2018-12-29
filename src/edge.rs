//! Functions for detecting edges in images.

use image::{GenericImageView, GrayImage, ImageBuffer, Luma, Rgba};
use imageproc::gradients::{horizontal_sobel, vertical_sobel};
use std::f32;

/// Runs the canny edge detection algorithm.
///
/// # Params
///
/// - `low_threshold`: Low threshold for the hysteresis procedure.
/// Edges with a strength higher than the low threshold will appear
/// in the output image, if there are strong edges nearby.
/// - `high_threshold`: High threshold for the hysteresis procedure.
/// Edges with a strength higher than the high threshold will always
/// appear as edges in the output image.
///
/// The greatest possible edge strength (and so largest sensible threshold)
/// is`sqrt(5) * 2 * 255`, or approximately 1140.39.
///
/// This odd looking value is the result of using a standard
/// definition of edge strength: the strength of an edge at a point `p` is
/// defined to be `sqrt(dx^2 + dy^2)`, where `dx` and `dy` are the values
/// of the horizontal and vertical Sobel gradients at `p`.
pub fn canny(
    image: &GrayImage,
    src_buf: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    low_threshold: f32,
    high_threshold: f32,
    count: u32,
) {
    assert!(high_threshold >= low_threshold);
    // Heavily based on the implementation proposed by wikipedia.
    // 1. Gaussian blur.(we don't do this step to boost speed).
    // const SIGMA: f32 = 1.4;
    // let blurred = gaussian_blur_f32(image, SIGMA);

    // 2. Intensity of gradients.
    let gx = horizontal_sobel(&image);
    let gy = vertical_sobel(&image);
    let g: Vec<f32> = gx
        .iter()
        .zip(gy.iter())
        .map(|(h, v)| (*h as f32).hypot(*v as f32))
        .collect::<Vec<f32>>();

    let g = ImageBuffer::from_raw(image.width(), image.height(), g).unwrap();

    // 3. Non-maximum-suppression (Make edges thinner)
    let thinned = non_maximum_suppression(&g, &gx, &gy);

    // 4. Hysteresis to filter out edges based on thresholds.
    hysteresis(&thinned, src_buf, low_threshold, high_threshold, count);
}

/// Finds local maxima to make the edges thinner.
fn non_maximum_suppression(
    g: &ImageBuffer<Luma<f32>, Vec<f32>>,
    gx: &ImageBuffer<Luma<i16>, Vec<i16>>,
    gy: &ImageBuffer<Luma<i16>, Vec<i16>>,
) -> ImageBuffer<Luma<f32>, Vec<f32>> {
    const RADIANS_TO_DEGREES: f32 = 180f32 / f32::consts::PI;
    let mut out = ImageBuffer::from_pixel(g.width(), g.height(), Luma { data: [0.0] });
    for y in 1..g.height() - 1 {
        for x in 1..g.width() - 1 {
            let x_gradient = gx[(x, y)][0] as f32;
            let y_gradient = gy[(x, y)][0] as f32;
            let mut angle = (y_gradient).atan2(x_gradient) * RADIANS_TO_DEGREES;
            if angle < 0.0 {
                angle += 180.0
            }
            // Clamp angle.
            let clamped_angle = if angle >= 157.5 || angle < 22.5 {
                0
            } else if angle >= 22.5 && angle < 67.5 {
                45
            } else if angle >= 67.5 && angle < 112.5 {
                90
            } else if angle >= 112.5 && angle < 157.5 {
                135
            } else {
                unreachable!()
            };

            // Get the two perpendicular neighbors.
            let (cmp1, cmp2) = unsafe {
                match clamped_angle {
                    0 => (g.unsafe_get_pixel(x - 1, y), g.unsafe_get_pixel(x + 1, y)),
                    45 => (
                        g.unsafe_get_pixel(x + 1, y + 1),
                        g.unsafe_get_pixel(x - 1, y - 1),
                    ),
                    90 => (g.unsafe_get_pixel(x, y - 1), g.unsafe_get_pixel(x, y + 1)),
                    135 => (
                        g.unsafe_get_pixel(x - 1, y + 1),
                        g.unsafe_get_pixel(x + 1, y - 1),
                    ),
                    _ => unreachable!(),
                }
            };
            let pixel = *g.get_pixel(x, y);
            // If the pixel is not a local maximum, suppress it.
            if pixel[0] < cmp1[0] || pixel[0] < cmp2[0] {
                out.put_pixel(x, y, Luma { data: [0.0] });
            } else {
                out.put_pixel(x, y, pixel);
            }
        }
    }
    out
}

/// Filter out edges with the thresholds.
/// Non-recursive breadth-first search.
fn hysteresis(
    input: &ImageBuffer<Luma<f32>, Vec<f32>>,
    out: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    low_thresh: f32,
    high_thresh: f32,
    hue: u32,
) {
    let (r, g, b) = cubehelix_to_rgb(hue, 1.0, 0.58);
    let pixel = image::Rgba {
        data: [r, g, b, 255u8],
    };
    // Init output image as all black.
    let mut tracking = ImageBuffer::from_pixel(input.width(), input.height(), Luma { data: [0u8] });
    // Stack. Possible optimization: Use previously allocated memory, i.e. gx.
    let mut edges = Vec::with_capacity(((input.width() * input.height()) / 2) as usize);
    for y in 1..input.height() - 1 {
        for x in 1..input.width() - 1 {
            let inp_pix = *input.get_pixel(x, y);
            let out_pix = *tracking.get_pixel(x, y);
            // If the edge strength is higher than high_thresh, mark it as an edge.
            if inp_pix[0] >= high_thresh && out_pix[0] == 0 {
                tracking.put_pixel(x, y, Luma { data: [255u8] });
                out.put_pixel(x, y, pixel);
                out.put_pixel(x - 1, y, pixel);
                out.put_pixel(x + 1, y, pixel);
                edges.push((x, y));

                // Track neighbors until no neighbor is >= low_thresh.
                while !edges.is_empty() {
                    let (nx, ny) = edges.pop().unwrap();
                    let neighbor_indices = [
                        (nx + 1, ny),
                        (nx + 1, ny + 1),
                        (nx, ny + 1),
                        (nx - 1, ny - 1),
                        (nx - 1, ny),
                        (nx - 1, ny + 1),
                    ];

                    for neighbor_idx in &neighbor_indices {
                        let in_neighbor = *input.get_pixel(neighbor_idx.0, neighbor_idx.1);
                        let out_neighbor = *tracking.get_pixel(neighbor_idx.0, neighbor_idx.1);
                        if in_neighbor[0] >= low_thresh && out_neighbor[0] == 0 {
                            tracking.put_pixel(
                                neighbor_idx.0,
                                neighbor_idx.1,
                                Luma { data: [255u8] },
                            );
                            out.put_pixel(neighbor_idx.0, neighbor_idx.1, pixel);
                            out.put_pixel(neighbor_idx.0 - 1, neighbor_idx.1, pixel);
                            out.put_pixel(neighbor_idx.0 + 1, neighbor_idx.1, pixel);
                            // out.put_pixel(neighbor_idx.0, neighbor_idx.1 - 1, pixel);
                            // out.put_pixel(neighbor_idx.0, neighbor_idx.1 + 1, pixel);
                            edges.push((neighbor_idx.0, neighbor_idx.1));
                        }
                    }
                }
            }
        }
    }
}

// Convert HSL to Cubehelix to RGB
const A: f32 = -0.14861_f32;
const B: f32 = 1.78277_f32;
const C: f32 = -0.29227_f32;
const D: f32 = -0.90649_f32;
const E: f32 = 1.97294_f32;
const PI: f32 = 3.14159265_f32;

fn hex(v: f32) -> u8 {
    if v <= 0.0 {
        0_u8
    } else if v >= 1.0 {
        255_u8
    } else {
        (v * 255.0).floor() as u8
    }
}

fn cubehelix_to_rgb(hue: u32, sat: f32, light: f32) -> (u8, u8, u8) {
    let h = (hue as f32 + 120.0) * (PI / 180.0);
    let l = light;
    let a = sat * light * (1.0 - light);
    let cosh = h.cos();
    let sinh = h.sin();
    return (
        hex(l + a * (A * cosh + B * sinh)),
        hex(l + a * (C * cosh + D * sinh)),
        hex(l + a * (E * cosh)),
    );
}
