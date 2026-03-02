use std::convert::TryFrom;

use byteorder::{ReadBytesExt, WriteBytesExt};

use crate::{convert::Convertable, Error, Result, ARTNET_PROTOCOL_VERSION};

data_structure! {
    #[derive(Debug)]
    #[doc = "Used to for timecode on the network"]
    pub struct Trigger {
        #[doc = "Determines which version the server has. Will be ARTNET_PROTOCOL_VERSION by default"]
        pub version: [u8; 2],

        #[doc = "Ignore by receiver, set to zero by sender"]
        pub filler1: u8,
        #[doc = "Ignore by receiver, set to zero by sender"]
        pub filler2: u8,
        #[doc = "The Oem code (high byte) of nodes that shall accept this trigger."]
        pub oem_hi: u8,
        #[doc = "The Oem code (low byte) of nodes that shall accept this trigger."]
        pub oem_lo: u8,
        #[doc = "The Trigger Key."]
        pub key: TriggerKey,
        #[doc = "The Trigger SubKey."]
        pub sub_key: u8,
        #[doc = "The interpretation of the payload is defined by the Key."]
        pub data: [u8; 512],
    }
}

impl Default for Trigger {
    fn default() -> Self {
        Self {
            version: ARTNET_PROTOCOL_VERSION,
            filler1: 0,
            filler2: 0,
            oem_hi: 0xff,
            oem_lo: 0xff,
            key: TriggerKey::Show,
            sub_key: 0,
            data: [0u8; 512],
        }
    }
}

/// The framerate being used for a particular [Timecode] stream.
#[repr(u8)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TriggerKey {
    #[doc = "The SubKey field contains an ASCII character which the receiving device should process as if it were a keyboard press."]
    Ascii = 0,
    #[doc = "The SubKey field contains the number of a Macro which the receiving device should execute."]
    Macro = 1,
    #[doc = "The SubKey field contains a soft-key number which the receiving device should process as if it were a soft-key keyboard press."]
    Soft = 2,
    #[doc = "The SubKey field contains the number of a Show which the receiving device should run."]
    Show = 3,
    #[doc = "Undefined"]
    Undefined(u8),
}

impl TryFrom<u8> for TriggerKey {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self> {
        let key = match value {
            0 => TriggerKey::Ascii,
            1 => TriggerKey::Macro,
            2 => TriggerKey::Soft,
            3 => TriggerKey::Show,
            other => TriggerKey::Undefined(other),
        };
        Ok(key)
    }
}

impl Into<u8> for TriggerKey {
    fn into(self) -> u8 {
        match self {
            TriggerKey::Ascii => 0,
            TriggerKey::Macro => 1,
            TriggerKey::Soft => 2,
            TriggerKey::Show => 3,
            TriggerKey::Undefined(other) => other,
        }
    }
}

impl<T> Convertable<T> for TriggerKey {
    fn from_cursor(cursor: &mut std::io::Cursor<&[u8]>) -> Result<Self> {
        let number = cursor.read_u8().map_err(Error::CursorEof)?;
        TriggerKey::try_from(number)
    }

    fn write_to_buffer(&self, buffer: &mut Vec<u8>, _context: &T) -> crate::Result<()> {
        let value = (*self).into();
        buffer.write_u8(value).map_err(Error::CursorEof)
    }

    #[cfg(test)]
    fn get_test_value() -> Self {
        TriggerKey::try_from(3).unwrap()
    }

    #[cfg(test)]
    fn is_equal(&self, other: &Self) -> bool {
        self == other
    }
}
