use std::{fs::File};

use image::{io::Reader, RgbImage};

use crate::tinybmp::*;

mod tinybmp;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let maybe_option = args.get(1).map(|s| s.as_str());
    let maybe_input_path = args.get(2).map(|s| s.as_str());
    let maybe_output_path = args.get(3).map(|s| s.as_str());

    match (maybe_option, maybe_input_path, maybe_output_path) {
        (Some("-e"), Some(input_path), Some(output_path)) =>
            encode(input_path, output_path).unwrap(),
        (Some("-d"), Some(input_path), Some(output_path)) =>
            decode(input_path, output_path).unwrap(),
        _ =>
            println!(
"Usage: tinybmp OPTION input-file output-file

Options:
  -e    encode an image into a tiny bitmap
  -d    decode a tiny bitmap into an image of the format determined by the file extension"
            ),
    }
}

fn encode(input_path: &str, output_path: &str) -> Result<()> {
    let img = Reader::open(input_path)?.decode()?;
    let rgb_img = img.into_rgb8();

    let mut file = File::create(output_path)?;

    let tiny_bmp: TinyBmp = rgb_img.try_into()?;
    tiny_bmp.write(&mut file)?;

    Ok(())
}

fn decode(input_path: &str, output_path: &str) -> Result<()> {
    let mut file = File::open(input_path)?;

    let tiny_bmp = TinyBmp::read(&mut file)?;
    let rgb_img: RgbImage = tiny_bmp.into();

    rgb_img.save(output_path)?;

    Ok(())
}
