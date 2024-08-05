mod color_schemas;
mod generator;
mod models;
mod parse;
use colored::Colorize;
use generator::generate;
use image::{png::PNGEncoder, ColorType};
use models::PlotColorSchema;
use parse::{parse_complex, parse_pair};
use std::{fs::File, str::FromStr};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 6 {
        eprintln!(
            "Usage: {} FILE PIXELS UPPER_LEFT LOWER_RIGHT COLOR_SCHEMA",
            args[0]
        );
        eprintln!(
            "Example: {} mandel.png 1000x750 -1.20,0.35 -1,0.20 palette/hue/log/cubic/linear/custom",
            args[0]
        );
        std::process::exit(1);
    }

    let bounds = parse_pair(&args[2], 'x').expect("error parsing image dimensions");
    let upper_left = parse_complex(&args[3]).expect("error parsing upper left corner point");
    let lower_right = parse_complex(&args[4]).expect("error parsing lower right corner point");
    let color_schema = PlotColorSchema::from_str(&args[5]).expect("error parsing color schema");
    let rows_per_band = bounds.1 / num_cpus::get() + 1;

    let pixels = generate(rows_per_band, bounds, upper_left, lower_right, color_schema);

    write_image(&args[1], &pixels, bounds, color_schema).expect("error writing PNG file");
}

fn write_image(
    filename: &str,
    pixels: &[u8],
    bounds: (usize, usize),
    color_schema: PlotColorSchema,
) -> Result<(), std::io::Error> {
    let schema = color_schema.to_string();
    let filename = filename.replace(".png", format!("_{}.png", schema).as_str());
    let output = File::create(&filename)?;
    let encoder = PNGEncoder::new(output);

    encoder.encode(pixels, bounds.0 as u32, bounds.1 as u32, ColorType::RGB(8))?;

    println!(
        "Image saved as {} with a color schema {}",
        filename.as_str().bold().yellow(),
        schema.as_str().green()
    );

    Ok(())
}
