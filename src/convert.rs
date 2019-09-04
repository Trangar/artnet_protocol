use crate::{Error, Result};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Cursor, Read};
use std::net::Ipv4Addr;

pub trait Convertable: Sized {
    fn from_cursor(cursor: &mut Cursor<&[u8]>) -> Result<Self>;
    fn into_buffer(&self, buffer: &mut Vec<u8>) -> Result<()>;
    fn get_test_value() -> Self;
    fn is_equal(&self, other: &Self) -> bool;
}

impl Convertable for Ipv4Addr {
    fn from_cursor(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(Ipv4Addr::new(
            cursor.read_u8().map_err(Error::CursorEof)?,
            cursor.read_u8().map_err(Error::CursorEof)?,
            cursor.read_u8().map_err(Error::CursorEof)?,
            cursor.read_u8().map_err(Error::CursorEof)?,
        ))
    }

    fn into_buffer(&self, buffer: &mut Vec<u8>) -> Result<()> {
        buffer.extend_from_slice(&self.octets());
        Ok(())
    }

    fn get_test_value() -> Self {
        Ipv4Addr::new(1, 2, 3, 4)
    }
    fn is_equal(&self, other: &Self) -> bool {
        self == other
    }
}

impl Convertable for Vec<u8> {
    fn from_cursor(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let remaining = cursor.get_ref();
        Ok(remaining[cursor.position() as usize..].to_vec())
    }

    fn into_buffer(&self, buffer: &mut Vec<u8>) -> Result<()> {
        buffer.extend_from_slice(&self[..]);
        Ok(())
    }
    fn get_test_value() -> Self {
        vec![1, 2, 3, 4]
    }
    fn is_equal(&self, other: &Self) -> bool {
        self == other
    }
}

impl Convertable for u8 {
    fn from_cursor(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        cursor.read_u8().map_err(Error::CursorEof)
    }

    fn into_buffer(&self, buffer: &mut Vec<u8>) -> Result<()> {
        buffer.push(*self);
        Ok(())
    }
    fn get_test_value() -> Self {
        1
    }
    fn is_equal(&self, other: &Self) -> bool {
        self == other
    }
}

macro_rules! convert_primitive {
    ([u8; $length:tt]) => {
        impl Convertable for [u8; $length] {
            fn from_cursor(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
                let mut result = [0; $length];
                cursor
                    .read_exact(&mut result[..])
                    .map_err(Error::CursorEof)?;
                Ok(result)
            }
            fn into_buffer(&self, buffer: &mut Vec<u8>) -> Result<()> {
                buffer.extend_from_slice(&self[..]);
                Ok(())
            }
            fn get_test_value() -> Self {
                [0; $length]
            }
            fn is_equal(&self, other: &Self) -> bool {
                &self[..] == &other[..]
            }
        }
    };
    ($ty:ty, $read_fn:tt, $write_fn:tt) => {
        impl Convertable for $ty {
            fn from_cursor(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
                cursor.$read_fn::<LittleEndian>().map_err(Error::CursorEof)
            }
            fn into_buffer(&self, buffer: &mut Vec<u8>) -> Result<()> {
                buffer
                    .$write_fn::<LittleEndian>(*self)
                    .map_err(Error::CursorEof)
            }
            fn get_test_value() -> Self {
                0
            }
            fn is_equal(&self, other: &Self) -> bool {
                self == other
            }
        }
    };
}

convert_primitive!(u16, read_u16, write_u16);
convert_primitive!([u8; 2]);
convert_primitive!([u8; 3]);
convert_primitive!([u8; 4]);
convert_primitive!([u8; 6]);
convert_primitive!([u8; 18]);
convert_primitive!([u8; 26]);
convert_primitive!([u8; 64]);
