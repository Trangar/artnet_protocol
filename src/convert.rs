use crate::{Error, Result};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Cursor, Read};
use std::net::Ipv4Addr;

pub trait Convertable<T>: Sized {
    fn from_cursor(cursor: &mut Cursor<&[u8]>) -> Result<Self>;
    fn write_to_buffer(&self, buffer: &mut Vec<u8>, context: &T) -> Result<()>;
    #[cfg(test)]
    fn get_test_value() -> Self;
    #[cfg(test)]
    fn is_equal(&self, other: &Self) -> bool;
}

impl<T> Convertable<T> for Ipv4Addr {
    fn from_cursor(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(Ipv4Addr::new(
            cursor.read_u8().map_err(Error::CursorEof)?,
            cursor.read_u8().map_err(Error::CursorEof)?,
            cursor.read_u8().map_err(Error::CursorEof)?,
            cursor.read_u8().map_err(Error::CursorEof)?,
        ))
    }

    fn write_to_buffer(&self, buffer: &mut Vec<u8>, _: &T) -> Result<()> {
        buffer.extend_from_slice(&self.octets());
        Ok(())
    }

    #[cfg(test)]
    fn get_test_value() -> Self {
        Ipv4Addr::new(1, 2, 3, 4)
    }
    #[cfg(test)]
    fn is_equal(&self, other: &Self) -> bool {
        self == other
    }
}

impl<T> Convertable<T> for Vec<u8> {
    fn from_cursor(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let remaining = cursor.get_ref();
        Ok(remaining[cursor.position() as usize..].to_vec())
    }

    fn write_to_buffer(&self, buffer: &mut Vec<u8>, _: &T) -> Result<()> {
        buffer.extend_from_slice(&self[..]);
        Ok(())
    }
    #[cfg(test)]
    fn get_test_value() -> Self {
        vec![1, 2, 3, 4]
    }
    #[cfg(test)]
    fn is_equal(&self, other: &Self) -> bool {
        self == other
    }
}

impl<T> Convertable<T> for u8 {
    fn from_cursor(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        cursor.read_u8().map_err(Error::CursorEof)
    }

    fn write_to_buffer(&self, buffer: &mut Vec<u8>, _: &T) -> Result<()> {
        buffer.push(*self);
        Ok(())
    }
    #[cfg(test)]
    fn get_test_value() -> Self {
        1
    }
    #[cfg(test)]
    fn is_equal(&self, other: &Self) -> bool {
        self == other
    }
}

macro_rules! convert_primitive {
    ([u8; $length:tt]) => {
        impl<T> Convertable<T> for [u8; $length] {
            fn from_cursor(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
                let mut result = [0; $length];
                cursor
                    .read_exact(&mut result[..])
                    .map_err(Error::CursorEof)?;
                Ok(result)
            }
            fn write_to_buffer(&self, buffer: &mut Vec<u8>, _: &T) -> Result<()> {
                buffer.extend_from_slice(&self[..]);
                Ok(())
            }
            #[cfg(test)]
            fn get_test_value() -> Self {
                [0; $length]
            }
            #[cfg(test)]
            fn is_equal(&self, other: &Self) -> bool {
                &self[..] == &other[..]
            }
        }
    };
    ($ty:ty, $read_fn:tt, $write_fn:tt) => {
        impl<T> Convertable<T> for $ty {
            fn from_cursor(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
                cursor.$read_fn::<LittleEndian>().map_err(Error::CursorEof)
            }
            fn write_to_buffer(&self, buffer: &mut Vec<u8>, _: &T) -> Result<()> {
                buffer
                    .$write_fn::<LittleEndian>(*self)
                    .map_err(Error::CursorEof)
            }
            #[cfg(test)]
            fn get_test_value() -> Self {
                0
            }
            #[cfg(test)]
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
