use anyhow::Context;
use image::GrayImage;
use num::Complex;
use num::complex::Complex64;

pub mod app_args;

pub fn escape_time(c: Complex64, limit: u32) -> Option<u32> {
    let mut z = Complex::new(0f64, 0f64);
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }

    None
}

fn pixel_to_point(
    (x_bound, y_bound): (usize, usize),
    (x, y): (usize, usize),
    upper_left: Complex64,
    lower_right: Complex64,
) -> Complex64 {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );
    Complex64::new(
        upper_left.re + x as f64 * width / x_bound as f64,
        upper_left.im - y as f64 * height / y_bound as f64,
    )
}

fn render(
    limit: u32,
    pixels: &mut [u8],
    bounds @ (x_bound, y_bound): (usize, usize),
    upper_left: Complex64,
    lower_right: Complex64,
) {
    assert_eq!(pixels.len(), x_bound * y_bound, "Wrong args");

    for row in 0..y_bound {
        for column in 0..x_bound {
            let point = pixel_to_point(bounds, (column, row), upper_left, lower_right);

            pixels[row * x_bound + column] = match escape_time(point, limit) {
                None => 0,
                Some(count) => 255 - count as u8,
            }
        }
    }
}

pub fn par_render(
    num_threads: usize,
    limit: u32,
    pixels: &mut [u8],
    bounds @ (x_bound, y_bound): (usize, usize),
    upper_left: Complex64,
    lower_right: Complex64,
) {
    let rows_ber_band = y_bound / num_threads + 1;
    let bands = pixels.chunks_mut(rows_ber_band * x_bound);

    rayon::scope(|spawner| {
        for (i, band) in bands.into_iter().enumerate() {
            let top = rows_ber_band * i;
            let height = band.len() / x_bound;
            let band_bounds = (x_bound, height);
            let band_upper_left = pixel_to_point(bounds, (0, top), upper_left, lower_right);
            let band_lower_right =
                pixel_to_point(bounds, (x_bound, top + height), upper_left, lower_right);

            spawner.spawn(move |_| {
                render(limit, band, band_bounds, band_upper_left, band_lower_right);
            });
        }
    })
}

pub fn write_image(
    path: &str,
    pixels: &[u8],
    (x_bound, y_bound): (usize, usize),
) -> anyhow::Result<()> {
    let buf = GrayImage::from_raw(x_bound as u32, y_bound as u32, pixels.to_vec()).unwrap();

    buf.save(path).context("Result image save")?;
    Ok(())
}
