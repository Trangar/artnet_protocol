use crate::ARTNET_PROTOCOL_VERSION;

data_structure! {
    #[derive(Debug)]
    #[doc = "Used to for timecode on the network"]
    pub struct Sync {
        #[doc = "Determines which version the server has. Will be ARTNET_PROTOCOL_VERSION by default"]
        pub version: [u8; 2],

        #[doc = "Transmit as zero."]
        pub aux1: u8,
        #[doc = "Transmit as zero."]
        pub aux2: u8,
    }
}

impl Default for Sync {
    fn default() -> Self {
        Self {
            version: ARTNET_PROTOCOL_VERSION,
            aux1: 0,
            aux2: 0,
        }
    }
}
