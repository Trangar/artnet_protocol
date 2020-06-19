use std::convert::TryFrom;

/// A `PortAddress` is an unsigned integer from 0 to 32_767 (15-bit).
///
/// The trait `From` is implemented for `u8`and `TryFrom` for `u16` and `i32`:
///
/// ```
/// use artnet_packer::PortAddress;
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
    type Error = String;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value <= 32_767 {
            Ok(PortAddress(value))
        } else {
            Err(format!(
                "Art-Net PortAddress must be from 0 to 32767. Got {}",
                value
            ))
        }
    }
}

// support un-annotated literals
impl TryFrom<i32> for PortAddress {
    type Error = String;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value <= 32_767 && value >= 0 {
            Ok(PortAddress(value as u16))
        } else {
            Err(format!(
                "Art-Net PortAddress must be from 0 to 32767. Got {}",
                value
            ))
        }
    }
}

impl PortAddress {
    #[allow(dead_code)]
    pub fn to_be_bytes(self) -> [u8; 2] {
        self.0.to_be_bytes()
    }
    pub fn to_le_bytes(self) -> [u8; 2] {
        self.0.to_le_bytes()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn port_address_to_bytes() {
        use std::convert::TryInto;
        let a: PortAddress = 0x1234.try_into().unwrap();
        assert!(a.to_be_bytes() == [0x12, 0x34]);
        assert!(a.to_le_bytes() == [0x34, 0x12]);
    }

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
