use std::convert::TryFrom;
use std::io::Cursor;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::{convert::Convertable, Error, Result};

/// A `PortAddress` is an unsigned integer from 0 to 32_767 (15-bit).
///
/// The trait `From` is implemented for `u8`and `TryFrom` for `u16` and `i32`:
///
/// ```
/// use artnet_protocol::PortAddress;
/// use std::convert::TryInto;
/// let a: PortAddress = 1.into(); //convert from u8 never fails
/// let b: PortAddress = 2u16.try_into().unwrap(); //u16 could fail if too big
/// let c: PortAddress = 3_000.try_into().unwrap(); //i32 could fail if too big or negative
/// //PortAddress of 0 is discouraged because sACN does not support a universe 0
/// let better_not = PortAddress::from(0);
/// ```
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct PortAddress(u16);

// basic support for u8 literals
impl From<u8> for PortAddress {
    fn from(value: u8) -> Self {
        // cannot over/underflow
        PortAddress(value as u16)
    }
}

impl TryFrom<u16> for PortAddress {
    type Error = Error;
    fn try_from(value: u16) -> Result<Self> {
        if value <= 32_767 {
            Ok(PortAddress(value))
        } else {
            Err(Error::InvalidPortAddress(value.into()))
        }
    }
}

// support un-annotated literals
impl TryFrom<i32> for PortAddress {
    type Error = Error;
    fn try_from(value: i32) -> Result<Self> {
        if (0..=32767).contains(&value) {
            Ok(PortAddress(value as u16))
        } else {
            Err(Error::InvalidPortAddress(value))
        }
    }
}

impl From<PortAddress> for u16 {
    fn from(value: PortAddress) -> Self {
        value.0
    }
}

impl<T> Convertable<T> for PortAddress {
    fn from_cursor(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let number = cursor
            .read_u16::<LittleEndian>()
            .map_err(Error::CursorEof)?;
        PortAddress::try_from(number)
    }

    fn write_to_buffer(&self, buffer: &mut Vec<u8>, _context: &T) -> Result<()> {
        buffer
            .write_u16::<LittleEndian>(self.0)
            .map_err(Error::CursorEof)
    }

    #[cfg(test)]
    fn get_test_value() -> Self {
        PortAddress::from(1)
    }

    #[cfg(test)]
    fn is_equal(&self, other: &Self) -> bool {
        self == other
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn port_address_bound_check() {
        use std::convert::TryInto;
        assert!(
            PortAddress::try_from(32_768u16).is_err(),
            "u16 values over 32_767 should not convert to PortAddress succesfully"
        );
        assert!(
            PortAddress::try_from(32_768).is_err(),
            "i32 values over 32_767 should not convert to PortAddress succesfully"
        );
        assert!(
            PortAddress::try_from(-1).is_err(),
            "negative i32 values should not convert to PortAddress succesfully"
        );
        assert!(
            PortAddress::try_from(-1_000).is_err(),
            "negative i32 values should not convert to PortAddress succesfully"
        );

        //should run without panic:
        let _c: PortAddress = 0.into();
        let _d: PortAddress = 255.into();
        let _e: PortAddress = 32_767.try_into().unwrap();
        let _f: PortAddress = 256.try_into().unwrap();
        let _f: PortAddress = 32_767u16.try_into().unwrap();
    }
}
