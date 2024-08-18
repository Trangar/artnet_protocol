use std::fmt;
use std::net::Ipv4Addr;
use std::str;

data_structure! {
    #[doc = "Gets send by the nodes in the network as a response to the Poll message"]
    pub struct PollReply {
        #[doc = "The IP address of the node"]
        pub address: Ipv4Addr,
        #[doc = "The port of the node, should always be 0x1936 / 6454"]
        pub port: u16,
        #[doc = "The version of the node"]
        pub version: [u8; 2],
        #[doc = "Bits 14-8 of the 15 bit Port-Address are encoded into the bottom 7 bits of the first byte. This is used in combination with SubSwitch and SwIn[] or SwOut[] to produce the full universe address."]
        #[doc = ""]
        #[doc = "Bits 7-4 of the 15 bit Port-Address are encoded into the bottom 4 bits of the second byte. This is used in combination with NetSwitch and SwIn[] or SwOut[] to produce the full universe address"]
        pub port_address: [u8; 2],
        #[doc = "The Oem word describes the equipment vendor and the feature set available. Bit 15 high indicates extended features available"]
        pub oem: [u8; 2],
        #[doc = "This field contains the firmware version of the User Bios Extension Area (UBEA). If the UBEA is not programmed, this field contains zero."]
        pub ubea_version: u8,
        #[doc = "General Status register. Will be expanded on in the future."]
        pub status_1: u8,
        #[doc = "The ESTA manufacturer code. These codes are used to represent equipment manufacturer. They are assigned by ESTA. This field can be interpreted as two ASCII bytes representing the manufacturer initials."]
        pub esta_code: u16,
        #[doc = "The array represents a null terminated short name for the Node. The Controller uses the ArtAddress packet to program this string. Max length is 17 characters plus the null. This is a fixed length field, although the string it contains can be shorter than the field."]
        pub short_name: [u8; 18],
        #[doc = "The array represents a null terminated long name for the Node. The Controller uses the ArtAddress packet to program this string. Max length is 63 characters plus the null. This is a fixed length field, although the string it contains can be shorter than the field."]
        pub long_name: [u8; 64],
        #[doc = "The array is a textual report of the Node’s operating status or operational errors. It is primarily intended for ‘engineering’ data rather than ‘end user’ data. The field is formatted as: “#xxxx [yyyy..] zzzzz…” xxxx is a hex status code as defined in Table 3. yyyy is a decimal counter that increments every time the Node sends an ArtPollResponse. This allows the controller to monitor event changes in the Node. zzzz is an English text string defining the status. This is a fixed length field, although the string it contains can be shorter than the field."]
        pub node_report: [u8; 64],
        #[doc = "The number of input or output ports. If number of inputs is not equal to number of outputs, the largest value is taken. Zero is a legal value if no input or output ports are implemented. The maximum value is 4. Nodes can ignore this field as the information is implicit in PortTypes[]"]
        pub num_ports: [u8; 2],
        #[doc = "This array defines the operation and protocol of each channel. (A product with 4 inputs and 4 outputs would report 0xc0, 0xc0, 0xc0, 0xc0). The array length is fixed, independent of the number of inputs or outputs physically available on the Node."]
        pub port_types: [u8; 4],
        #[doc = "This array defines input status of the node. Will be converted to a `bitflag` enum in the future."]
        pub good_input: [u8; 4],
        #[doc = "This array defines output status of the node. Will be converted to a `bitflag` enum in the future."]
        pub good_output: [u8; 4],
        #[doc = "Bits 3-0 of the 15 bit Port-Address for each of the 4 possible input ports are encoded into the low nibble"]
        pub swin: [u8; 4],
        #[doc = "Bits 3-0 of the 15 bit Port-Address for each of the 4 possible output ports are encoded into the low nibble."]
        pub swout: [u8; 4],
        #[doc = "Set to 00 when video display is showing local data. Set to 01 when video is showing ethernet data. The field is now deprecated"]
        pub sw_video: u8,
        #[doc = "If the Node supports macro key inputs, this byte represents the trigger values. The Node is responsible for ‘debouncing’ inputs. When the ArtPollReply is set to transmit automatically, (TalkToMe Bit 1), the ArtPollReply will be sent on both key down and key up events. However, the Controller should not assume that only one bit position has changed. The Macro inputs are used for remote event triggering or cueing. "]
        pub sw_macro: u8,
        #[doc = "If the Node supports remote trigger inputs, this byte represents the trigger values. The Node is responsible for ‘debouncing’ inputs. When the ArtPollReply is set to transmit automatically, (TalkToMe Bit 1), the ArtPollReply will be sent on both key down and key up events. However, the Controller should not assume that only one bit position has changed. The Remote inputs are used for remote event triggering or cueing."]
        pub sw_remote: u8,
        #[doc(hidden)]
        pub spare: [u8; 3],
        #[doc = "The Style code defines the equipment style of the device."]
        pub style: u8,
        #[doc = "MAC Address. Set to zero if node cannot supply this information."]
        pub mac: [u8; 6],
        #[doc = "If this unit is part of a larger or modular product, this is the IP of the root device"]
        pub bind_ip: [u8; 4],
        #[doc = "This number represents the order of bound devices. A lower number means closer to root device. A value of 1 means root device"]
        pub bind_index: u8,
        #[doc = "Status 2. Will be expanded in the future"]
        pub status_2: u8,
        #[doc = "Transmit as zero. For future expansion."]
        pub filler: [u8; 26],
    }
}

impl fmt::Debug for PollReply {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let short_name = str::from_utf8(&self.short_name)
            .map(String::from)
            .unwrap_or_else(|e| format!("Invalid UTF8: {:?}", e));
        let long_name = str::from_utf8(&self.long_name)
            .map(String::from)
            .unwrap_or_else(|e| format!("Invalid UTF8: {:?}", e));

        fmt.debug_struct("PollReply")
            .field("address", &self.address)
            .field("port", &self.port)
            .field("version", &self.version)
            .field("port_address", &self.port_address)
            .field("oem", &self.oem)
            .field("ubea_version", &self.ubea_version)
            .field("status_1", &self.status_1)
            .field("esta_code", &self.esta_code)
            .field("short_name", &short_name.trim_end_matches('\0'))
            .field("long_name", &long_name.trim_end_matches('\0'))
            .field("node_report", &&self.node_report[..])
            .field("num_ports", &self.num_ports)
            .field("port_types", &self.port_types)
            .field("good_input", &self.good_input)
            .field("good_output", &self.good_output)
            .field("swin", &self.swin)
            .field("swout", &self.swout)
            .field("sw_video", &self.sw_video)
            .field("sw_macro", &self.sw_macro)
            .field("sw_remote", &self.sw_remote)
            .field("style", &self.style)
            .field("mac", &self.mac)
            .field("bind_ip", &self.bind_ip)
            .field("bind_index", &self.bind_index)
            .field("filler", &self.filler)
            .finish()
    }
}

impl Default for PollReply {
    fn default() -> Self {
        // Per Art-Net spec, unused fields are zero
        PollReply {
            address: Ipv4Addr::from_bits(0),
            port: 6454,
            version: [0; 2],
            port_address: [0; 2],
            oem: [0; 2],
            ubea_version: 0,
            status_1: 0,
            esta_code: 0,
            short_name: [0; 18],
            long_name: [0; 64],
            node_report: [0; 64],
            num_ports: [0; 2],
            port_types: [0; 4],
            good_input: [0; 4],
            good_output: [0; 4],
            swin: [0; 4],
            swout: [0; 4],
            sw_video: 0,
            sw_macro: 0,
            sw_remote: 0,
            spare: [0; 3],
            style: 0,
            mac: [0; 6],
            bind_ip: [0; 4],
            bind_index: 0,
            status_2: 0,
            filler: [0; 26],
        }
    }
}
