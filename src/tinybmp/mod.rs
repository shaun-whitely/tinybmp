use std::{io::{Write, Read}};

use image::{Rgb, RgbImage};

use self::error::{ConversionError, ReadError};

mod interop;
pub mod error;

pub struct TinyBmp {
    palate: [Rgb<u8>; 4],
    width: u8,
    data: Vec<u8>,
}

impl TryFrom<RgbImage> for TinyBmp {
    type Error = ConversionError;

    fn try_from(value: RgbImage) -> Result<Self, Self::Error> {
        interop::from_rgb_image(&value)
    }
}

impl From<TinyBmp> for RgbImage {
    fn from(value: TinyBmp) -> Self {
        interop::to_rgb_image(&value)
    }
}

impl TinyBmp {
    pub fn read<R>(reader: &mut R) -> Result<TinyBmp, ReadError> where R: Read {
        let mut magic_bytes = [0u8; 2];
        reader.read(&mut magic_bytes)?;

        if magic_bytes != [0x54u8, 0x69u8] {
            return Err(ReadError::UnrecognizedFormat);
        }

        let mut depth = [0u8];
        reader.read(&mut depth)?;

        if depth[0] != 3 {
            return Err(ReadError::UnsupportedColorDepth(depth[0]))
        }

        let mut color1 = [0u8; 3];
        let mut color2 = [0u8; 3];
        let mut color3 = [0u8; 3];
        let mut color4 = [0u8; 3];

        reader.read(&mut color1)?;
        reader.read(&mut color2)?;
        reader.read(&mut color3)?;
        reader.read(&mut color4)?;

        let mut width = [0u8];

        reader.read(&mut width)?;

        let mut compressed_data: Vec<u8> = Vec::new();
        let mut data: Vec<u8> = Vec::new();

        reader.read_to_end(&mut compressed_data)?;

        for compressed_pixels in compressed_data {
            let p1 = (compressed_pixels & 0b11000000) >> 6;
            let p2 = (compressed_pixels & 0b00110000) >> 4;
            let p3 = (compressed_pixels & 0b00001100) >> 2;
            let p4 = compressed_pixels & 0b00000011;

            let mut pixels = vec!(p1, p2, p3, p4);

            data.append(&mut pixels);
        }

        let palate = [
            Rgb(color1),
            Rgb(color2),
            Rgb(color3),
            Rgb(color4),
        ];

        let result = TinyBmp { palate: palate, width: width[0], data: data};

        Ok(result)
    }

    pub fn write<W>(&self, writer: &mut W) -> Result<(), std::io::Error> where W: Write {
        let magic_bytes = [0x54u8, 0x69u8];
        let color_depth = 0x03u8;

        writer.write(&magic_bytes)?;
        writer.write(&[color_depth])?;

        let palate_bytes =
            self.palate
                .map(|color| color.0)
                .concat();

        writer.write(&palate_bytes)?;
        writer.write(&[self.width])?;

        let mut i: usize = 0;
        while i < self.data.len() {
            let p1 = self.data[i];
            let p2 = self.data[i + 1];
            let p3 = self.data[i + 2];
            let p4 = self.data[i + 3];

            let compressed_pixel =
                (p1 << 6) | (p2 << 4) | (p3 << 2) | p4;

            writer.write(&[compressed_pixel])?;

            i += 4;
        }

        Ok(())
    }
}
