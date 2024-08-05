use crate::{
    color_schemas::{
        color_cubic_polynomial, color_custom, color_linear_gradient, color_logarithmic_mapping,
        color_palette, hue_rotation,
    },
    models::PlotColorSchema,
};
use num::Complex;

pub fn generate(
    rows_per_band: usize,
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
    color_schema: PlotColorSchema,
) -> Vec<u8> {
    let mut pixels = vec![0; bounds.0 * bounds.1 * 3];

    let bands: Vec<&mut [u8]> = pixels.chunks_mut(rows_per_band * bounds.0 * 3).collect();

    crossbeam::scope(|spawner| {
        for (i, band) in bands.into_iter().enumerate() {
            let top = rows_per_band * i;
            let height = band.len() / (bounds.0 * 3);
            let band_bounds = (bounds.0, height);
            let band_upper_left = pixel_to_point(bounds, (0, top), upper_left, lower_right);
            let band_lower_right =
                pixel_to_point(bounds, (bounds.0, top + height), upper_left, lower_right);

            spawner.spawn(move |_| {
                render(
                    band,
                    band_bounds,
                    band_upper_left,
                    band_lower_right,
                    color_schema,
                );
            });
        }
    })
    .unwrap();

    pixels
}

fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );

    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
    }
}

fn render(
    pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
    color_schema: PlotColorSchema,
) {
    assert!(pixels.len() == bounds.0 * bounds.1 * 3);

    for row in 0..bounds.1 {
        for col in 0..bounds.0 {
            let point = pixel_to_point(bounds, (col, row), upper_left, lower_right);
            let color = escape_time_to_color(escape_time(point, 255), 255, color_schema);
            let pixel_index = (row * bounds.0 + col) * 3;

            pixels[pixel_index] = color[0];
            pixels[pixel_index + 1] = color[1];
            pixels[pixel_index + 2] = color[2];
        }
    }
}

fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };

    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }

        z = z * z + c;
    }

    None
}

fn escape_time_to_color(
    count: Option<usize>,
    limit: usize,
    color_schema: PlotColorSchema,
) -> [u8; 3] {
    match count {
        None => [0, 0, 0], //black
        Some(count) => {
            let t = count as f32 / limit as f32;
            match color_schema {
                PlotColorSchema::Palette => color_palette(count),
                PlotColorSchema::Custom => color_custom(t),
                PlotColorSchema::CubicPolynomial => color_cubic_polynomial(t),
                PlotColorSchema::HueRotation => hue_rotation(t),
                PlotColorSchema::LinearGradient => color_linear_gradient(t),
                PlotColorSchema::LogarithmicMapping => color_logarithmic_mapping(count, limit),
            }
        }
    }
}
