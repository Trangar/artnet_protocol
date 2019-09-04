use crate::byteorder::ReadBytesExt;
use crate::convert::Convertable;
use crate::{Error, Result};
use std::io::Cursor;

bitflags! {
    /// The TalkToMe flag, as to be used in the `Poll` and `PollReply` message
    pub struct ArtTalkToMe: u8 {
        /// Enable VLC transmission if set, disabled otherwise
        const ENABLE_VLC = 0b0001_0000;

        /// Diagnostic messages are unicast. If this is not set, the messages are broadcast. Has no effect if `ENABLE_DIAGNOSTICS` is not set.
        const UNICAST_DIAGNOSTICS = 0b0000_1000;

        /// Enable diagnostics
        const ENABLE_DIAGNOSTICS = 0b0000_0100;

        /// Configure the nodes to send ArtPollReply whenever something changes on their end. If this is not set, the devices will only send ArtPollReply if ArtPoll is send.
        const EMIT_CHANGES = 0b0000_0010;

        /// No flags
        const NONE = 0b0000_0000;
    }
}

impl Convertable for ArtTalkToMe {
    fn from_cursor(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let b = cursor.read_u8().map_err(Error::CursorEof)?;
        Ok(ArtTalkToMe::from_bits_truncate(b))
    }
    fn into_buffer(&self, buffer: &mut Vec<u8>) -> Result<()> {
        buffer.push(self.bits());
        Ok(())
    }
    fn get_test_value() -> Self {
        ArtTalkToMe::NONE
    }
    fn is_equal(&self, other: &Self) -> bool {
        self == other
    }
}
