use std::convert::TryFrom;

use byteorder::{ReadBytesExt, WriteBytesExt};

use crate::{convert::Convertable, Error, Result};

data_structure! {
    #[derive(Debug)]
    #[doc = "Used to for timecode on the network"]
    pub struct Timecode {
        #[doc = "Determines which version the server has. Will be ARTNET_PROTOCOL_VERSION by default"]
        pub version: [u8; 2],

        // #[doc = "Determines how the nodes should respond"]
        // pub talk_to_me: ArtTalkToMe,

        // #[doc = "Determines the priority of the diagnostics that the nodes should send"]
        // pub diagnostics_priority: u8,

        #[doc = "Ignore by receiver, set to zero by sender"]
        pub filler1: u8,
        #[doc = "Used to identify different streams of time code. Value of 0x00 is the master"]
        pub stream_id: u8,
        #[doc = "Frames time. 0 – 29 depending on mode"]
        pub frames: u8,
        #[doc = "Seconds. 0 - 59"]
        pub seconds: u8,
        #[doc = "Minutes. 0 - 59"]
        pub minutes: u8,
        #[doc = "Hours. 0 - 23"]
        pub hours: u8,
        #[doc = "FrameType "]
        pub frame_type: FrameType,
    }
}

#[repr(u8)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum FrameType {
    #[doc = "FrameType Film 24fps"]
    Film = 0,
    #[doc = "FrameType EBU 25fps"]
    #[allow(clippy::upper_case_acronyms)]
    EBU = 1,
    #[doc = "FrameType DF 29.97fps"]
    #[allow(clippy::upper_case_acronyms)]
    DF = 2,
    #[doc = "FrameType SMPTE 30fps"]
    #[allow(clippy::upper_case_acronyms)]
    SMPTE = 3,
}

impl TryFrom<u8> for FrameType {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self> {
        match value {
            0 => Ok(FrameType::Film),
            1 => Ok(FrameType::EBU),
            2 => Ok(FrameType::DF),
            3 => Ok(FrameType::SMPTE),
            _ => Err(Error::InvalidTimecodeFrameType(value)),
        }
    }
}

impl<T> Convertable<T> for FrameType {
    fn from_cursor(cursor: &mut std::io::Cursor<&[u8]>) -> Result<Self> {
        let number = cursor.read_u8().map_err(Error::CursorEof)?;
        FrameType::try_from(number)
    }

    fn write_to_buffer(&self, buffer: &mut Vec<u8>, _context: &T) -> crate::Result<()> {
        let value = *self as u8;
        buffer.write_u8(value).map_err(Error::CursorEof)
    }

    #[cfg(test)]
    fn get_test_value() -> Self {
        FrameType::try_from(1).unwrap()
    }

    #[cfg(test)]
    fn is_equal(&self, other: &Self) -> bool {
        self == other
    }
}
