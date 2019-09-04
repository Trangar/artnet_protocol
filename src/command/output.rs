use crate::command::ARTNET_PROTOCOL_VERSION;

data_structure! {
    #[derive(Debug)]
    #[doc = "ArtDmx is the data packet used to transfer DMX512 data. The format is identical for Node to Controller, Node to Node and Controller to Node."]
    #[doc = ""]
    #[doc = "The Data is output through the DMX O/P port corresponding to the Universe setting. In the absence of received ArtDmx packets, each DMX O/P port re-transmits the same frame continuously. "]
    #[doc = ""]
    #[doc = "The first complete DMX frame received at each input port is placed in an ArtDmx packet as above and transmitted as an ArtDmx packet containing the relevant Universe parameter. Each subsequent DMX frame containing new data (different length or different contents) is also transmitted as an ArtDmx packet."]
    #[doc = ""]
    #[doc = "Nodes do not transmit ArtDmx for DMX512 inputs that have not received data since power on."]
    #[doc = ""]
    #[doc = "However, an input that is active but not changing, will re-transmit the last valid ArtDmx packet at approximately 4-second intervals. (Note. In order to converge the needs of ArtNet and sACN it is recommended that Art-Net devices actually use a re-transmit time of 800mS to 1000mS)."]
    #[doc = ""]
    #[doc = "A DMX input that fails will not continue to transmit ArtDmx data."]
    pub struct Output {
        #[doc = "Determines which version the server has. Will be ARTNET_PROTOCOL_VERSION by default"]
        pub version: [u8; 2],
        #[doc = "The sequence number is used to ensure that ArtDmx packets are used in the correct order. When Art-Net is carried over a medium such as the Internet, it is possible that ArtDmx packets will reach the receiver out of order. This field is incremented in the range 0x01 to 0xff to allow the receiving node to resequence packets."]
        #[doc = ""]
        #[doc = "The Sequence field is set to 0x00 to disable this feature"]
        pub sequence: u8,
        #[doc = "The physical input port from which DMX512 data was input. This field is for information only. Use Universe for data routing"]
        pub physical: u8,
        #[doc = "The 15 bit Port-Address to which this packet is destined"]
        pub subnet: u16,
        #[doc = "The length of the DMX512 data array. This value should be an even number in the range 2 – 512."]
        #[doc = ""]
        #[doc = "It represents the number of DMX512 channels encoded in packet. NB: Products which convert Art-Net to DMX512 may opt to always send 512 channels."]
        pub length: u16,
        #[doc = "A variable length array of DMX512 lighting data"]
        pub data: Vec<u8>,
    }
}

impl Default for Output {
    fn default() -> Output {
        Output {
            version: ARTNET_PROTOCOL_VERSION,
            sequence: 0,
            physical: 0,
            subnet: 0,
            length: 0,
            data: Vec::new(),
        }
    }
}
