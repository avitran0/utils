use std::{io::Result, mem::size_of};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Endian {
    Native,
    Little,
    Big,
}

pub trait ReadBytes: std::io::Read {
    #[inline]
    fn read_u8(&mut self) -> Result<u8> {
        let mut buf = [0; 1];
        self.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    #[inline]
    fn read_u16(&mut self) -> Result<u16> {
        let mut buf = [0; 2];
        self.read_exact(&mut buf)?;
        Ok(u16::from_ne_bytes(buf))
    }

    #[inline]
    fn read_u16_endian(&mut self, endian: Endian) -> Result<u16> {
        let mut buf = [0; 2];
        self.read_exact(&mut buf)?;
        Ok(match endian {
            Endian::Native => u16::from_ne_bytes(buf),
            Endian::Little => u16::from_le_bytes(buf),
            Endian::Big => u16::from_be_bytes(buf),
        })
    }

    #[inline]
    fn read_u32(&mut self) -> Result<u32> {
        let mut buf = [0; 4];
        self.read_exact(&mut buf)?;
        Ok(u32::from_ne_bytes(buf))
    }

    #[inline]
    fn read_u32_endian(&mut self, endian: Endian) -> Result<u32> {
        let mut buf = [0; 4];
        self.read_exact(&mut buf)?;
        Ok(match endian {
            Endian::Native => u32::from_ne_bytes(buf),
            Endian::Little => u32::from_le_bytes(buf),
            Endian::Big => u32::from_be_bytes(buf),
        })
    }

    #[inline]
    fn read_u64(&mut self) -> Result<u64> {
        let mut buf = [0; 8];
        self.read_exact(&mut buf)?;
        Ok(u64::from_ne_bytes(buf))
    }

    #[inline]
    fn read_u64_endian(&mut self, endian: Endian) -> Result<u64> {
        let mut buf = [0; 8];
        self.read_exact(&mut buf)?;
        Ok(match endian {
            Endian::Native => u64::from_ne_bytes(buf),
            Endian::Little => u64::from_le_bytes(buf),
            Endian::Big => u64::from_be_bytes(buf),
        })
    }

    #[inline]
    fn read_u128(&mut self) -> Result<u128> {
        let mut buf = [0; 16];
        self.read_exact(&mut buf)?;
        Ok(u128::from_ne_bytes(buf))
    }

    #[inline]
    fn read_u128_endian(&mut self, endian: Endian) -> Result<u128> {
        let mut buf = [0; 16];
        self.read_exact(&mut buf)?;
        Ok(match endian {
            Endian::Native => u128::from_ne_bytes(buf),
            Endian::Little => u128::from_le_bytes(buf),
            Endian::Big => u128::from_be_bytes(buf),
        })
    }

    #[inline]
    fn read_i8(&mut self) -> Result<i8> {
        let mut buf = [0; 1];
        self.read_exact(&mut buf)?;
        Ok(buf[0].cast_signed())
    }

    #[inline]
    fn read_i16(&mut self) -> Result<i16> {
        let mut buf = [0; 2];
        self.read_exact(&mut buf)?;
        Ok(i16::from_ne_bytes(buf))
    }

    #[inline]
    fn read_i16_endian(&mut self, endian: Endian) -> Result<i16> {
        let mut buf = [0; 2];
        self.read_exact(&mut buf)?;
        Ok(match endian {
            Endian::Native => i16::from_ne_bytes(buf),
            Endian::Little => i16::from_le_bytes(buf),
            Endian::Big => i16::from_be_bytes(buf),
        })
    }

    #[inline]
    fn read_i32(&mut self) -> Result<i32> {
        let mut buf = [0; 4];
        self.read_exact(&mut buf)?;
        Ok(i32::from_ne_bytes(buf))
    }

    #[inline]
    fn read_i32_endian(&mut self, endian: Endian) -> Result<i32> {
        let mut buf = [0; 4];
        self.read_exact(&mut buf)?;
        Ok(match endian {
            Endian::Native => i32::from_ne_bytes(buf),
            Endian::Little => i32::from_le_bytes(buf),
            Endian::Big => i32::from_be_bytes(buf),
        })
    }

    #[inline]
    fn read_i64(&mut self) -> Result<i64> {
        let mut buf = [0; 8];
        self.read_exact(&mut buf)?;
        Ok(i64::from_ne_bytes(buf))
    }

    #[inline]
    fn read_i64_endian(&mut self, endian: Endian) -> Result<i64> {
        let mut buf = [0; 8];
        self.read_exact(&mut buf)?;
        Ok(match endian {
            Endian::Native => i64::from_ne_bytes(buf),
            Endian::Little => i64::from_le_bytes(buf),
            Endian::Big => i64::from_be_bytes(buf),
        })
    }

    #[inline]
    fn read_i128(&mut self) -> Result<i128> {
        let mut buf = [0; 16];
        self.read_exact(&mut buf)?;
        Ok(i128::from_ne_bytes(buf))
    }

    #[inline]
    fn read_i128_endian(&mut self, endian: Endian) -> Result<i128> {
        let mut buf = [0; 16];
        self.read_exact(&mut buf)?;
        Ok(match endian {
            Endian::Native => i128::from_ne_bytes(buf),
            Endian::Little => i128::from_le_bytes(buf),
            Endian::Big => i128::from_be_bytes(buf),
        })
    }

    #[inline]
    fn read_f32(&mut self) -> Result<f32> {
        let mut buf = [0; 4];
        self.read_exact(&mut buf)?;
        Ok(f32::from_ne_bytes(buf))
    }

    #[inline]
    fn read_f32_endian(&mut self, endian: Endian) -> Result<f32> {
        let mut buf = [0; 4];
        self.read_exact(&mut buf)?;
        Ok(match endian {
            Endian::Native => f32::from_ne_bytes(buf),
            Endian::Little => f32::from_le_bytes(buf),
            Endian::Big => f32::from_be_bytes(buf),
        })
    }

    #[inline]
    fn read_f64(&mut self) -> Result<f64> {
        let mut buf = [0; 8];
        self.read_exact(&mut buf)?;
        Ok(f64::from_ne_bytes(buf))
    }

    #[inline]
    fn read_f64_endian(&mut self, endian: Endian) -> Result<f64> {
        let mut buf = [0; 8];
        self.read_exact(&mut buf)?;
        Ok(match endian {
            Endian::Native => f64::from_ne_bytes(buf),
            Endian::Little => f64::from_le_bytes(buf),
            Endian::Big => f64::from_be_bytes(buf),
        })
    }

    #[inline]
    fn read_bytes(&mut self, count: usize) -> Result<Vec<u8>> {
        let mut buf = vec![0; count];
        self.read_exact(&mut buf)?;
        Ok(buf)
    }

    #[inline]
    fn read_value<T: Default>(&mut self) -> Result<T> {
        let mut value = T::default();
        let buf = core::slice::from_mut(&mut value);
        let buf = unsafe {
            core::slice::from_raw_parts_mut::<u8>(buf.as_mut_ptr().cast(), size_of::<T>())
        };
        self.read_exact(buf)?;
        Ok(value)
    }
}

impl<R: std::io::Read> ReadBytes for R {}

pub trait WriteBytes: std::io::Write {
    #[inline]
    fn write_u8(&mut self, value: u8) -> Result<()> {
        self.write_all(&[value])
    }

    #[inline]
    fn write_u16(&mut self, value: u16) -> Result<()> {
        self.write_all(&value.to_ne_bytes())
    }

    #[inline]
    fn write_u16_endian(&mut self, value: u16, endian: Endian) -> Result<()> {
        let bytes = match endian {
            Endian::Native => value.to_ne_bytes(),
            Endian::Little => value.to_le_bytes(),
            Endian::Big => value.to_be_bytes(),
        };
        self.write_all(&bytes)
    }

    #[inline]
    fn write_u32(&mut self, value: u32) -> Result<()> {
        self.write_all(&value.to_ne_bytes())
    }

    #[inline]
    fn write_u32_endian(&mut self, value: u32, endian: Endian) -> Result<()> {
        let bytes = match endian {
            Endian::Native => value.to_ne_bytes(),
            Endian::Little => value.to_le_bytes(),
            Endian::Big => value.to_be_bytes(),
        };
        self.write_all(&bytes)
    }

    #[inline]
    fn write_u64(&mut self, value: u64) -> Result<()> {
        self.write_all(&value.to_ne_bytes())
    }

    #[inline]
    fn write_u64_endian(&mut self, value: u64, endian: Endian) -> Result<()> {
        let bytes = match endian {
            Endian::Native => value.to_ne_bytes(),
            Endian::Little => value.to_le_bytes(),
            Endian::Big => value.to_be_bytes(),
        };
        self.write_all(&bytes)
    }

    #[inline]
    fn write_u128(&mut self, value: u128) -> Result<()> {
        self.write_all(&value.to_ne_bytes())
    }

    #[inline]
    fn write_u128_endian(&mut self, value: u128, endian: Endian) -> Result<()> {
        let bytes = match endian {
            Endian::Native => value.to_ne_bytes(),
            Endian::Little => value.to_le_bytes(),
            Endian::Big => value.to_be_bytes(),
        };
        self.write_all(&bytes)
    }

    #[inline]
    fn write_i8(&mut self, value: i8) -> Result<()> {
        self.write_all(&value.to_ne_bytes())
    }

    #[inline]
    fn write_i16(&mut self, value: i16) -> Result<()> {
        self.write_all(&value.to_ne_bytes())
    }

    #[inline]
    fn write_i16_endian(&mut self, value: i16, endian: Endian) -> Result<()> {
        let bytes = match endian {
            Endian::Native => value.to_ne_bytes(),
            Endian::Little => value.to_le_bytes(),
            Endian::Big => value.to_be_bytes(),
        };
        self.write_all(&bytes)
    }

    #[inline]
    fn write_i32(&mut self, value: i32) -> Result<()> {
        self.write_all(&value.to_ne_bytes())
    }

    #[inline]
    fn write_i32_endian(&mut self, value: i32, endian: Endian) -> Result<()> {
        let bytes = match endian {
            Endian::Native => value.to_ne_bytes(),
            Endian::Little => value.to_le_bytes(),
            Endian::Big => value.to_be_bytes(),
        };
        self.write_all(&bytes)
    }

    #[inline]
    fn write_i64(&mut self, value: i64) -> Result<()> {
        self.write_all(&value.to_ne_bytes())
    }

    #[inline]
    fn write_i64_endian(&mut self, value: i64, endian: Endian) -> Result<()> {
        let bytes = match endian {
            Endian::Native => value.to_ne_bytes(),
            Endian::Little => value.to_le_bytes(),
            Endian::Big => value.to_be_bytes(),
        };
        self.write_all(&bytes)
    }

    #[inline]
    fn write_i128(&mut self, value: i128) -> Result<()> {
        self.write_all(&value.to_ne_bytes())
    }

    #[inline]
    fn write_i128_endian(&mut self, value: i128, endian: Endian) -> Result<()> {
        let bytes = match endian {
            Endian::Native => value.to_ne_bytes(),
            Endian::Little => value.to_le_bytes(),
            Endian::Big => value.to_be_bytes(),
        };
        self.write_all(&bytes)
    }

    #[inline]
    fn write_f32(&mut self, value: f32) -> Result<()> {
        self.write_all(&value.to_ne_bytes())
    }

    #[inline]
    fn write_f32_endian(&mut self, value: f32, endian: Endian) -> Result<()> {
        let bytes = match endian {
            Endian::Native => value.to_ne_bytes(),
            Endian::Little => value.to_le_bytes(),
            Endian::Big => value.to_be_bytes(),
        };
        self.write_all(&bytes)
    }

    #[inline]
    fn write_f64(&mut self, value: f64) -> Result<()> {
        self.write_all(&value.to_ne_bytes())
    }

    #[inline]
    fn write_f64_endian(&mut self, value: f64, endian: Endian) -> Result<()> {
        let bytes = match endian {
            Endian::Native => value.to_ne_bytes(),
            Endian::Little => value.to_le_bytes(),
            Endian::Big => value.to_be_bytes(),
        };
        self.write_all(&bytes)
    }

    #[inline]
    fn write_bytes(&mut self, bytes: &[u8]) -> Result<()> {
        self.write_all(bytes)
    }

    #[inline]
    fn write_value<T: Copy>(&mut self, value: &T) -> Result<()> {
        let buf = core::slice::from_ref(value);
        let buf = unsafe { core::slice::from_raw_parts::<u8>(buf.as_ptr().cast(), size_of::<T>()) };
        self.write_all(buf)
    }
}

impl<W: std::io::Write> WriteBytes for W {}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use super::{Endian, ReadBytes, WriteBytes};

    #[test]
    fn read_write_native_roundtrip() {
        let mut cursor = Cursor::new(Vec::new());

        cursor.write_u32(0x1234_5678).unwrap();
        cursor.write_i16(-1234).unwrap();
        cursor.write_f32(3.5).unwrap();

        cursor.set_position(0);

        assert_eq!(cursor.read_u32().unwrap(), 0x1234_5678);
        assert_eq!(cursor.read_i16().unwrap(), -1234);
        assert_eq!(cursor.read_f32().unwrap(), 3.5);
    }

    #[test]
    fn read_write_endian_roundtrip() {
        let mut cursor = Cursor::new(Vec::new());

        cursor.write_u32_endian(0x0102_0304, Endian::Big).unwrap();
        cursor.write_u16_endian(0x0506, Endian::Little).unwrap();

        cursor.set_position(0);

        assert_eq!(cursor.read_u32_endian(Endian::Big).unwrap(), 0x0102_0304);
        assert_eq!(cursor.read_u16_endian(Endian::Little).unwrap(), 0x0506);
    }
}
