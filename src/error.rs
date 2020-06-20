use std::ops::Range;

/// The result that this crate uses
pub type Result<T> = std::result::Result<T, Error>;

/// All the possible errors this crate can encounter
#[derive(Debug)]
pub enum Error {
    /// Could not read or write to the inner curso
    CursorEof(std::io::Error),

    /// Could not serialize an artnet command
    SerializeError(&'static str, Box<Error>),

    /// Could not deserialize an artnet command
    DeserializeError(&'static str, Box<Error>),

    /// The given message was too short
    MessageTooShort {
        /// The message that was being send or received
        message: Vec<u8>,

        /// The minimal length that is supported
        min_len: usize,
    },

    /// The given message was too long or too short
    MessageSizeInvalid {
        /// The message that was being send or received
        message: Vec<u8>,

        /// The size that the artnet protocol expects
        allowed_size: Range<usize>,
    },

    /// The artnet header is invalid
    InvalidArtnetHeader(Vec<u8>),

    /// Could not parse the given opcode
    OpcodeError(&'static str, Box<Error>),

    /// Unknown opcode ID
    UnknownOpcode(u16),
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::CursorEof(inner) => write!(fmt, "Cursor EOF: {}", inner),
            Error::SerializeError(message, inner) => write!(fmt, "{}: {}", message, inner),
            Error::DeserializeError(message, inner) => write!(fmt, "{}: {}", message, inner),
            Error::MessageTooShort { message, min_len } => write!(
                fmt,
                "Message too short, it was {} but artnet expects at least {}",
                message.len(),
                min_len
            ),
            Error::MessageSizeInvalid {
                message,
                allowed_size,
            } => write!(
                fmt,
                "Message size invalid, it was {} but artnet expects between {} and {}",
                message.len(),
                allowed_size.start,
                allowed_size.end
            ),
            Error::InvalidArtnetHeader(_) => write!(fmt, "Invalid artnet header"),
            Error::OpcodeError(opcode, inner) => {
                write!(fmt, "Could not parse opcode {:?}: {}", opcode, inner)
            }
            Error::UnknownOpcode(opcode) => write!(fmt, "Unknown opcode 0x{:X}", opcode),
        }
    }
}

impl std::error::Error for Error {}
