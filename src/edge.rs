//! Functions for detecting edges in images.

use image::{GenericImage, GenericImageView, GrayImage, ImageBuffer, Luma, Rgba};
// use imageproc::filter::gaussian_blur_f32;
use std::cmp::{max, min};
use std::mem::transmute;
use std::sync::Mutex;
use std::{f32, i16};

static BLACK_32: Luma<f32> = Luma { data: [0.0] };

/// Sobel filter for detecting vertical gradients.
static VERTICAL_SOBEL: [i32; 9] = [-1, -2, -1, 0, 0, 0, 1, 2, 1];

/// Sobel filter for detecting horizontal gradients.
static HORIZONTAL_SOBEL: [i32; 9] = [-1, 0, 1, -2, 0, 2, -1, 0, 1];

lazy_static! {
    // Since it's mutable and shared, use mutext.
    static ref VERT:  Mutex<ImageBuffer<Luma<i16>, Vec<i16>>> = Mutex::new(ImageBuffer::new(640, 480));
    static ref HORIZ:  Mutex<ImageBuffer<Luma<i16>, Vec<i16>>> = Mutex::new(ImageBuffer::new(640, 480));
    static ref IN_IMAGE: Mutex<ImageBuffer<Luma<f32>, Vec<f32>>> = Mutex::new(ImageBuffer::new(640, 480));
    static ref OUT_IMAGE: Mutex<ImageBuffer<Luma<f32>, Vec<f32>>> = Mutex::new(ImageBuffer::new(640, 480));
    static ref EDGES: Mutex<Vec<(u32, u32)>> = Mutex::new(Vec::with_capacity((640 * 480) / 2));
}

/// Runs the canny edge detection algorithm.
///
/// # Params
///
/// - `low_threshold`: Low threshold for the hysteresis procedure.  Edges with a strength higher than the low threshold will appear
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
    hue: u32,
    use_thick: bool,
) {
    let mut gx = HORIZ.lock().unwrap();
    let mut gy = VERT.lock().unwrap();
    let mut out = OUT_IMAGE.lock().unwrap();
    let mut in_image = IN_IMAGE.lock().unwrap();

    // Heavily based on the implementation proposed by wikipedia.
    // 1. Gaussian blur.(we don't do this step to boost speed).
    // const SIGMA: f32 = 1.4;
    // let blurred = gaussian_blur_f32(image, SIGMA);

    // 2. Intensity of gradients.
    filter(
        &image,
        HORIZONTAL_SOBEL,
        VERTICAL_SOBEL,
        &mut gx,
        &mut gy,
        &mut in_image,
    );

    // 3. Non-maximum-suppression (Make edges thinner)
    non_maximum_suppression(&in_image, &gx, &gy, &mut out);

    // 4. Hysteresis to filter out edges based on thresholds.
    hysteresis(&out, src_buf, low_threshold, high_threshold, hue, use_thick);
}

/// Finds local maxima to make the edges thinner.
fn non_maximum_suppression(
    g: &ImageBuffer<Luma<f32>, Vec<f32>>,
    gx: &ImageBuffer<Luma<i16>, Vec<i16>>,
    gy: &ImageBuffer<Luma<i16>, Vec<i16>>,
    out: &mut ImageBuffer<Luma<f32>, Vec<f32>>,
) {
    const RADIANS_TO_DEGREES: f32 = 180f32 / f32::consts::PI;
    for y in 1..g.height() - 1 {
        for x in 1..g.width() - 1 {
            let x_gradient = gx[(x, y)][0] as f32;
            let y_gradient = gy[(x, y)][0] as f32;
            let mut angle = atan2_approx(y_gradient, x_gradient) * RADIANS_TO_DEGREES;
            if angle < 0.0 {
                angle += 180.0
            }
            let (cmp1, cmp2) = unsafe {
                if angle >= 157.5 || angle < 22.5 {
                    (g.unsafe_get_pixel(x - 1, y), g.unsafe_get_pixel(x + 1, y))
                } else if angle >= 22.5 && angle < 67.5 {
                    (
                        g.unsafe_get_pixel(x + 1, y + 1),
                        g.unsafe_get_pixel(x - 1, y - 1),
                    )
                } else if angle >= 67.5 && angle < 112.5 {
                    (g.unsafe_get_pixel(x, y - 1), g.unsafe_get_pixel(x, y + 1))
                } else if angle >= 112.5 && angle < 157.5 {
                    (
                        g.unsafe_get_pixel(x - 1, y + 1),
                        g.unsafe_get_pixel(x + 1, y - 1),
                    )
                } else {
                    unreachable!()
                }
            };

            unsafe {
                let pixel = g.unsafe_get_pixel(x, y);
                // If the pixel is not a local maximum, suppress it.
                if pixel[0] < cmp1[0] || pixel[0] < cmp2[0] {
                    out.unsafe_put_pixel(x, y, BLACK_32);
                } else {
                    out.unsafe_put_pixel(x, y, pixel);
                }
            }
        }
    }
}

/// Filter out edges with the thresholds.
/// Non-recursive breadth-first search.
fn hysteresis(
    input: &ImageBuffer<Luma<f32>, Vec<f32>>,
    out: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    low_thresh: f32,
    high_thresh: f32,
    hue: u32,
    use_thick: bool,
) {
    let color: [u8; 4] = unsafe { transmute(hue.to_be()) };
    let low_thresh = low_thresh * low_thresh;
    let high_thresh = high_thresh * high_thresh;
    let pixel = image::Rgba { data: color };
    // Init output image as all black.
    let mut edges = EDGES.lock().unwrap();
    edges.clear();

    for y in 1..input.height() - 1 {
        for x in 1..input.width() - 1 {
            let (inp_pix, out_pix) =
                unsafe { (input.unsafe_get_pixel(x, y), out.unsafe_get_pixel(x, y)) };
            // If the edge strength is higher than high_thresh, mark it as an edge.
            if inp_pix[0] >= high_thresh && out_pix[0] != pixel[0] {
                unsafe {
                    out.unsafe_put_pixel(x, y, pixel);
                };
                edges.push((x, y));

                // Track neighbors until no neighbor is >= low_thresh.
                while !edges.is_empty() {
                    let (nx, ny) = edges.pop().unwrap();
                    let neighbor_indices = [
                        (nx + 1, ny, nx - 1, ny),
                        (nx + 1, ny + 1, nx - 1, ny + 1),
                        (nx, ny + 1, nx + 1, ny),
                        (nx - 1, ny - 1, nx + 1, ny + 1),
                        (nx - 1, ny, nx, ny + 1),
                        (nx - 1, ny + 1, nx - 1, ny - 1),
                    ];
                    // let neighbor_indices = [
                    //     (nx + 1, ny),
                    //     (nx + 1, ny + 1),
                    //     (nx, ny + 1),
                    //     (nx - 1, ny - 1),
                    //     (nx - 1, ny),
                    //     (nx - 1, ny + 1),
                    // ];

                    for neighbor_idx in &neighbor_indices {
                        let (in_neighbor, out_neighbor) = unsafe {
                            (
                                input.unsafe_get_pixel(neighbor_idx.0, neighbor_idx.1),
                                out.unsafe_get_pixel(neighbor_idx.0, neighbor_idx.1),
                            )
                        };
                        if in_neighbor[0] >= low_thresh && out_neighbor[0] != pixel[0] {
                            unsafe {
                                out.unsafe_put_pixel(neighbor_idx.0, neighbor_idx.1, pixel);
                                if use_thick {
                                    out.unsafe_put_pixel(neighbor_idx.2, neighbor_idx.3, pixel);
                                }
                            };
                            edges.push((neighbor_idx.0, neighbor_idx.1));
                        }
                    }
                }
            }
        }
    }
}

pub fn filter(
    image: &GrayImage,
    hdata: [i32; 9],
    vdata: [i32; 9],
    hout: &mut ImageBuffer<Luma<i16>, Vec<i16>>,
    vout: &mut ImageBuffer<Luma<i16>, Vec<i16>>,
    out: &mut ImageBuffer<Luma<f32>, Vec<f32>>,
) {
    let (width, height) = image.dimensions();
    let (k_width, k_height) = (3, 3);

    let mut hacc = 0_i32;
    let mut vacc = 0_i32;

    for y in 0..height {
        for x in 0..width {
            for k_y in 0..k_height {
                let y_p = min(
                    height + height - 1,
                    max(height, height + y + k_y - k_height / 2),
                ) - height;
                for k_x in 0..k_width {
                    let x_p =
                        min(width + width - 1, max(width, width + x + k_x - k_width / 2)) - width;
                    let (p, hk, vk) = unsafe {
                        (
                            image.unsafe_get_pixel(x_p, y_p),
                            hdata.get_unchecked((k_y * k_width + k_x) as usize),
                            vdata.get_unchecked((k_y * k_width + k_x) as usize),
                        )
                    };
                    hacc = accumulate(hacc, &p, *hk);
                    vacc = accumulate(vacc, &p, *vk);
                }
            }

            let h = clamp(hacc);
            let v = clamp(vacc);
            let p = (h as f32) * (h as f32) + (v as f32) * (v as f32);
            hacc = 0_i32;
            vacc = 0_i32;
            unsafe {
                hout.unsafe_put_pixel(x, y, Luma { data: [h] });
                vout.unsafe_put_pixel(x, y, Luma { data: [v] });
                out.unsafe_put_pixel(x, y, Luma { data: [p] });
            };
        }
    }
}

fn clamp(x: i32) -> i16 {
    if x < i16::MAX as i32 {
        if x > i16::MIN as i32 {
            x as i16
        } else {
            i16::MIN
        }
    } else {
        i16::MAX
    }
}

fn accumulate(acc: i32, pixel: &Luma<u8>, weight: i32) -> i32 {
    acc + (pixel.data[0] as i32) * weight
}

// borrowed this code from: https://gist.github.com/volkansalma/2972237
pub fn atan2_approx(y: f32, x: f32) -> f32 {
    const ONEQTR_PI: f32 = f32::consts::PI / 4.0;
    const THRQTR_PI: f32 = 3.0 * f32::consts::PI / 4.0;
    let abs_y = (y).abs() + 1e-10_f32;
    let (r, angle) = if x < 0.0 {
        ((x + abs_y) / (abs_y - x), THRQTR_PI)
    } else {
        ((x - abs_y) / (x + abs_y), ONEQTR_PI)
    };
    let angle = angle + (0.1963 * r * r - 0.9817) * r;
    if y < 0.0 {
        -angle
    } else {
        angle
    }
}
