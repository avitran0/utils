use std::io::{Read, Write};

use crate::io::{ReadBytes, WriteBytes};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

pub struct Image {
    width: u32,
    height: u32,
    pixels: Vec<Pixel>,
}

impl Image {
    const MAGIC: [u8; 4] = [b'P', b'I', b'M', b'G'];
    const MAGIC_U32: u32 = u32::from_le_bytes(Self::MAGIC);

    pub fn new(width: u32, height: u32) -> Self {
        let pixels = Vec::with_capacity(width as usize * height as usize);
        Self {
            width,
            height,
            pixels,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, pixel: Pixel) {
        let index = self.pixel_index(x, y);
        if let Some(p) = self.pixels.get_mut(index) {
            *p = pixel;
        }
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Option<&Pixel> {
        let index = self.pixel_index(x, y);
        self.pixels.get(index)
    }

    fn pixel_index(&self, x: u32, y: u32) -> usize {
        y as usize * self.height as usize + x as usize
    }

    pub fn read(reader: &mut impl Read) -> std::io::Result<Self> {
        let magic = reader.read_u32()?;

        if magic != Self::MAGIC_U32 {
            return Err(std::io::Error::other(format!(
                "Magic mismatch: 0x{magic:8X} != 0x{:8X}",
                Self::MAGIC_U32
            )));
        }

        let width = reader.read_u32()?;
        let height = reader.read_u32()?;

        let pixel_count = width as usize * height as usize;
        let pixels = reader.read_value_vec(pixel_count)?;

        Ok(Self {
            width,
            height,
            pixels,
        })
    }

    pub fn write(&self, writer: &mut impl Write) -> std::io::Result<()> {
        writer.write_u32(Self::MAGIC_U32)?;

        writer.write_u32(self.width)?;
        writer.write_u32(self.height)?;

        writer.write_value_vec(&self.pixels)?;

        Ok(())
    }
}
