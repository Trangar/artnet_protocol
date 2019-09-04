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

    /// The given message was not long enough
    MessageTooShort(Vec<u8>),

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
            Error::MessageTooShort(_) => write!(fmt, "Message too short"),
            Error::InvalidArtnetHeader(_) => write!(fmt, "Invalid artnet header"),
            Error::OpcodeError(opcode, inner) => {
                write!(fmt, "Could not parse opcode {:?}: {}", opcode, inner)
            }
            Error::UnknownOpcode(opcode) => write!(fmt, "Unknown opcode 0x{:X}", opcode),
        }
    }
}

impl std::error::Error for Error {}
