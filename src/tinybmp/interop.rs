use image::{RgbImage, Rgb};

use super::{TinyBmp, error::ConversionError};

pub fn to_rgb_image(tiny_bmp: &TinyBmp) -> RgbImage {
    let width = tiny_bmp.width as u32;
    let height = tiny_bmp.data.len() as u32 / width;

    let mut rgb_image = RgbImage::new(width, height);

    let zipped_iter = tiny_bmp.data.iter().zip(rgb_image.pixels_mut());

    for (&pixel, out_pixel) in zipped_iter {
        let color = tiny_bmp.palate[pixel as usize];
        *out_pixel = color;
    }

    rgb_image
}

pub fn from_rgb_image(rgb_img: &RgbImage) -> Result<TinyBmp, ConversionError> {
    let (width, height) = rgb_img.dimensions();

    let valid_width: u8 = width.try_into().map_err(|_| ConversionError::TooWide(width))?;

    let mut palate: [Option<&Rgb<u8>>; 4] = [None; 4];

    let mut output: Vec<u8> = vec![0; (width * height) as usize];

    let zipped_iter = rgb_img.pixels().zip(output.iter_mut());

    for (pixel, out_pixel) in zipped_iter {
        let maybe_index = palate_index(palate.as_slice(), pixel);

        let index = match maybe_index {
            Some(index) => index,
            None => {
                let new_index = palate
                    .iter_mut()
                    .position(|o| o.is_none())
                    .ok_or(ConversionError::TooManyColors)?;

                palate[new_index] = Some(pixel);
                new_index
            },
        };

        *out_pixel = index as u8;
    }

    let complete_palate = palate.map(|color|
        color.cloned().unwrap_or(Rgb([0, 0, 0]))
    );

    let tiny_bmp =
        TinyBmp { palate: complete_palate, width: valid_width, data: output };

    Ok(tiny_bmp)
}

fn palate_index(palate: &[Option<&Rgb<u8>>], pixel: &Rgb<u8>) -> Option<usize> {
    for (i, maybe_color) in palate.iter().enumerate() {
        if *maybe_color == Some(pixel) {
            return Some(i);
        }
    }

    None
}
