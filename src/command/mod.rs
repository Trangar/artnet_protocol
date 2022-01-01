mod output;
mod poll;
mod poll_reply;

use crate::{Error, Result};
use byteorder::{ByteOrder, LittleEndian, WriteBytesExt};

pub use self::output::{Output, PaddedData};
pub use self::poll::Poll;
pub use self::poll_reply::PollReply;

/// The ArtCommand, to be used for ArtNet.
///
/// This struct implements an `write_to_buffer` and `from_buffer` function, to be used with UDP connections.

#[derive(Debug)]
pub enum ArtCommand {
    /// A poll command, used to discover devices on the network
    Poll(Poll),

    /// A reply to the poll command, it contains device status information
    PollReply(Box<PollReply>),

    /// [Not implemented] Diagnostics and data logging packet
    DiagData,

    /// [Not implemented] Used to send text based parameter commands
    Command,

    /// An ArtDmx data packet. Used to send actual data to a node in the network
    Output(Output),

    /// [Not implemented] This is an ArtNzs data packet. It contains non-zero start code (except RDM) DMX512 information for a single Universe
    Nzs,

    /// [Not implemented] This is an ArtSync data packet. It is used to force synchronous transfer of ArtDmx packets to a node's output
    Sync,

    /// [Not implemented] This is an ArtAddress packet. It contains remote programming information for a Node.
    Address,

    /// [Not implemented] This is an ArtInput packet. It contains enable – disable data for DMX inputs
    Input,

    /// [Not implemented] This is an ArtTodRequest packet. It is used to request a Table of Devices (ToD) for RDM discovery.
    TodRequest,

    /// [Not implemented] This is an ArtTodData packet. It is used to send a Table of Devices (ToD) for RDM discovery
    TodData,

    /// [Not implemented] This is an ArtTodControl packet. It is used to send RDM discovery control messages.
    TodControl,

    /// [Not implemented] This is an ArtRdm packet. It is used to send all non discovery RDM messages
    Rdm,

    /// [Not implemented] This is an ArtRdmSub packet. It is used to send compressed, RDM Sub-Device data.
    RdmSub,

    /// [Not implemented] This is an ArtVideoSetup packet. It contains video screen setup information for nodes that implement the extended video features.
    VideoSetup,

    /// [Not implemented] This is an ArtVideoPalette packet. It contains colour palette setup information for nodes that implement the extended video features.
    VideoPalette,

    /// [Not implemented] This is an ArtVideoData packet. It contains display data for nodes that implement the extended video features.
    VideoData,

    /// [Not implemented] This packet is deprecated
    MacMaster,

    /// [Not implemented] This packet is deprecated
    MacSlave,

    /// [Not implemented] This is an ArtFirmwareMaster packet. It is used to upload new firmware or firmware extensions to the Node.
    FirmwareMaster,

    /// [Not implemented] This is an ArtFirmwareReply packet. It is returned by the node to acknowledge receipt of an ArtFirmwareMaster packet or ArtFileTnMaster packet.
    FirmwareReply,

    /// [Not implemented] Uploads user file to node.
    FileTnMaster,

    /// [Not implemented] Downloads user file from node
    FileFnMaster,

    /// [Not implemented] Server to Node acknowledge for download packets
    FileFnReply,

    /// [Not implemented] This is an ArtIpProg packet. It is used to reprogramme the IP address and Mask of the Node
    OpIpProg,

    /// [Not implemented] This is an ArtIpProgReply packet. It is returned by the node to acknowledge receipt of an ArtIpProg packet.
    OpIpProgReply,

    /// [Not implemented] This is an ArtMedia packet. It is Unicast by a Media Server and acted upon by a Controller
    OpMedia,

    /// [Not implemented] This is an ArtMediaPatch packet. It is Unicast by a Controller and acted upon by a Media Server
    OpMediaPatch,

    /// [Not implemented] This is an ArtMediaControl packet. It is Unicast by a Controller and acted upon by a Media Server.
    OpMediaControl,

    /// [Not implemented] This is an ArtMediaControlReply packet. It is Unicast by a Media Server and acted upon by a Controller
    OpMediaControlReply,

    /// [Not implemented] This is an ArtTimeCode packet. It is used to transport time code over the network
    OpTimeCode,

    /// [Not implemented] Used to synchronise real time date and clock
    OpTimeSync,

    /// [Not implemented] Used to send trigger macros
    OpTrigger,

    /// [Not implemented] Requests a node's file list
    OpDirectory,

    /// [Not implemented] Replies to OpDirectory with file list
    OpDirectoryReply,
}

/// The ArtNet header. This is the first 8 bytes of each message, and contains the text "Art-Net\0"
pub const ARTNET_HEADER: &[u8; 8] = b"Art-Net\0";

/// The protocol version. Anything above [4, 0] seems to work for the devices that this library was tested on.
///
/// If you need a different or configurable protocol version, please open a PR.
pub const ARTNET_PROTOCOL_VERSION: [u8; 2] = [0, 14];

impl ArtCommand {
    /// Convert an ArtCommand in a byte buffer, which can be send to an UDP socket.
    pub fn write_to_buffer(self) -> Result<Vec<u8>> {
        let mut result = Vec::new();
        let (opcode, data) = self.get_opcode()?;

        // Append Art-Net\0 header
        result.extend_from_slice(ARTNET_HEADER);
        // Append the opcode of this enum
        result
            .write_u16::<LittleEndian>(opcode)
            .map_err(Error::CursorEof)?;

        result.extend_from_slice(&data);

        Ok(result)
    }

    /// Convert an a byte buffer to a command.
    pub fn from_buffer(buffer: &[u8]) -> Result<ArtCommand> {
        const MIN_BUFFER_LENGTH: usize = 14;

        if buffer.len() < MIN_BUFFER_LENGTH {
            return Err(Error::MessageTooShort {
                message: buffer.to_vec(),
                min_len: MIN_BUFFER_LENGTH,
            });
        }

        if !buffer.starts_with(ARTNET_HEADER) {
            return Err(Error::InvalidArtnetHeader(buffer.to_vec()));
        }

        let opcode = LittleEndian::read_u16(&buffer[8..10]);
        let remaining = &buffer[10..];

        let command = ArtCommand::opcode_to_enum(opcode, remaining)?;

        Ok(command)
    }

    fn opcode_to_enum(code: u16, data: &[u8]) -> Result<ArtCommand> {
        Ok(match code {
            0x2000 => ArtCommand::Poll(
                Poll::from(data).map_err(|e| Error::OpcodeError("Poll", Box::new(e)))?,
            ),
            0x2100 => ArtCommand::PollReply(Box::new(
                PollReply::from(data).map_err(|e| Error::OpcodeError("PollReply", Box::new(e)))?,
            )),
            0x2300 => ArtCommand::DiagData,
            0x2400 => ArtCommand::Command,
            0x5000 => ArtCommand::Output(
                Output::from(data).map_err(|e| Error::OpcodeError("Output", Box::new(e)))?,
            ),
            0x5100 => ArtCommand::Nzs,
            0x5200 => ArtCommand::Sync,
            0x6000 => ArtCommand::Address,
            0x7000 => ArtCommand::Input,
            0x8000 => ArtCommand::TodRequest,
            0x8100 => ArtCommand::TodData,
            0x8200 => ArtCommand::TodControl,
            0x8300 => ArtCommand::Rdm,
            0x8400 => ArtCommand::RdmSub,
            0xA010 => ArtCommand::VideoSetup,
            0xA020 => ArtCommand::VideoPalette,
            0xA040 => ArtCommand::VideoData,
            0xF000 => ArtCommand::MacMaster,
            0xF100 => ArtCommand::MacSlave,
            0xF200 => ArtCommand::FirmwareMaster,
            0xF300 => ArtCommand::FirmwareReply,
            0xF400 => ArtCommand::FileTnMaster,
            0xF500 => ArtCommand::FileFnMaster,
            0xF600 => ArtCommand::FileFnReply,
            0xF800 => ArtCommand::OpIpProg,
            0xF900 => ArtCommand::OpIpProgReply,
            0x9000 => ArtCommand::OpMedia,
            0x9100 => ArtCommand::OpMediaPatch,
            0x9200 => ArtCommand::OpMediaControl,
            0x9300 => ArtCommand::OpMediaControlReply,
            0x9700 => ArtCommand::OpTimeCode,
            0x9800 => ArtCommand::OpTimeSync,
            0x9900 => ArtCommand::OpTrigger,
            0x9A00 => ArtCommand::OpDirectory,
            0x9B00 => ArtCommand::OpDirectoryReply,
            _ => return Err(Error::UnknownOpcode(code)),
        })
    }

    fn get_opcode(&self) -> Result<(u16, Vec<u8>)> {
        Ok(match self {
            ArtCommand::Poll(poll) => (0x2000, poll.to_bytes()?),
            ArtCommand::PollReply(reply) => (0x2100, reply.to_bytes()?),
            ArtCommand::DiagData => (0x2300, Vec::new()),
            ArtCommand::Command => (0x2400, Vec::new()),
            ArtCommand::Output(output) => (0x5000, output.to_bytes()?),
            ArtCommand::Nzs => (0x5100, Vec::new()),
            ArtCommand::Sync => (0x5200, Vec::new()),
            ArtCommand::Address => (0x6000, Vec::new()),
            ArtCommand::Input => (0x7000, Vec::new()),
            ArtCommand::TodRequest => (0x8000, Vec::new()),
            ArtCommand::TodData => (0x8100, Vec::new()),
            ArtCommand::TodControl => (0x8200, Vec::new()),
            ArtCommand::Rdm => (0x8300, Vec::new()),
            ArtCommand::RdmSub => (0x8400, Vec::new()),
            ArtCommand::VideoSetup => (0xA010, Vec::new()),
            ArtCommand::VideoPalette => (0xA020, Vec::new()),
            ArtCommand::VideoData => (0xA040, Vec::new()),
            ArtCommand::MacMaster => (0xF000, Vec::new()),
            ArtCommand::MacSlave => (0xF100, Vec::new()),
            ArtCommand::FirmwareMaster => (0xF200, Vec::new()),
            ArtCommand::FirmwareReply => (0xF300, Vec::new()),
            ArtCommand::FileTnMaster => (0xF400, Vec::new()),
            ArtCommand::FileFnMaster => (0xF500, Vec::new()),
            ArtCommand::FileFnReply => (0xF600, Vec::new()),
            ArtCommand::OpIpProg => (0xF800, Vec::new()),
            ArtCommand::OpIpProgReply => (0xF900, Vec::new()),
            ArtCommand::OpMedia => (0x9000, Vec::new()),
            ArtCommand::OpMediaPatch => (0x9100, Vec::new()),
            ArtCommand::OpMediaControl => (0x9200, Vec::new()),
            ArtCommand::OpMediaControlReply => (0x9300, Vec::new()),
            ArtCommand::OpTimeCode => (0x9700, Vec::new()),
            ArtCommand::OpTimeSync => (0x9800, Vec::new()),
            ArtCommand::OpTrigger => (0x9900, Vec::new()),
            ArtCommand::OpDirectory => (0x9A00, Vec::new()),
            ArtCommand::OpDirectoryReply => (0x9B00, Vec::new()),
        })
    }
}
