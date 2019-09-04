use crate::ArtTalkToMe;

data_structure! {
    #[derive(Debug)]
    #[doc = "Used to poll the nodes in the network"]
    pub struct Poll {
        #[doc = "Determines which version the server has. Will be ARTNET_PROTOCOL_VERSION by default"]
        pub version: [u8; 2],

        #[doc = "Determines how the nodes should respond"]
        pub talk_to_me: ArtTalkToMe,

        #[doc = "Determines the priority of the diagnostics that the nodes should send"]
        pub diagnostics_priority: u8,
    }
}

impl Default for Poll {
    fn default() -> Poll {
        Poll {
            version: super::ARTNET_PROTOCOL_VERSION,
            talk_to_me: ArtTalkToMe::NONE,
            diagnostics_priority: 0x80,
        }
    }
}
