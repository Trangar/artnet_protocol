use super::*;
use crate::ArtCommand;

mod serialization {
    use super::*;
    #[test]
    fn create_single_dmx_value_art_dmx_packet() {
        let command = ArtCommand::Output(Output {
            data: vec![255].into(), // The data we're sending to the node
            ..Output::default()
        });
        let bytes = command.write_to_buffer().unwrap();
        let comparison = vec![
            65, 114, 116, 45, 78, 101, 116, 0, 0, 80, 0, 14, 0, 0, 1, 0, 0, 2, 255, 0,
        ]; //is padded with zero to even length of two
        assert_eq!(bytes, comparison)
    }
    #[test]
    fn create_512_dmx_values_art_dmx_packet() {
        let command = ArtCommand::Output(Output {
            data: vec![128; 512].into(), // The data we're sending to the node
            ..Output::default()
        });
        let bytes = command.write_to_buffer().unwrap();
        let comparison = [
            vec![
                65, 114, 116, 45, 78, 101, 116, 0, 0, 80, 0, 14, 0, 0, 1, 0, 2, 0,
            ],
            vec![128; 512],
        ]
        .concat(); //is padded with zero to even length of two
        assert_eq!(bytes, comparison)
    }
    #[test]
    fn test_invalid_length() {
        let command = ArtCommand::Output(Output {
            data: vec![0xff; 512].into(),
            ..Output::default()
        });
        let buffer = command.write_to_buffer().unwrap();
        // #6: length needs to be encoded in big endian
        assert_eq!(&buffer[0x10..=0x11], &[2, 0]);
        // #7.1: packets need to be an even number
        fn get_data(command: &ArtCommand) -> &PaddedData {
            if let ArtCommand::Output(output) = command {
                &output.data
            } else {
                unreachable!()
            }
        }
        let command = ArtCommand::Output(Output {
            data: vec![0xff].into(),
            ..Output::default()
        });
        // Initially it will be 1
        assert_eq!(get_data(&command).len(), 1);
        // But the padded length is 2
        assert_eq!(get_data(&command).len_rounded_up(), 2);
        let buffer = command.write_to_buffer().unwrap();
        // The data written is 2 bytes
        assert_eq!(&buffer[0x10..=0x11], &[0, 2]);
        // #7.2: packets need to be at least 2 bytes
        let command = ArtCommand::Output(Output {
            data: vec![].into(),
            ..Output::default()
        });
        assert!(command.write_to_buffer().is_err());
        // #7.3: packets need to be at most 512 bytes
        let command = ArtCommand::Output(Output {
            data: vec![0xff; 513].into(),
            ..Output::default()
        });
        assert!(command.write_to_buffer().is_err());
    }
}

mod parsing {
    use super::*;

    #[test]
    fn protver_below_14() {
        // Because Art-Net is guaranteed to be backwards-compatible,
        // we should be able to parse versions below 14,
        // even tough these should never be seen in the wild
        let packet = &[
            65, 114, 116, 45, 78, 101, 116, 0, 0, 80, 0, 0, 0, 0, 1, 0, 0, 2, 255, 255,
        ];
        let command = ArtCommand::from_buffer(packet).unwrap();
        if let ArtCommand::Output(output) = command {
            assert_eq!(output.version, [0, 0]);
            assert_eq!(output.sequence, 0);
            assert_eq!(output.physical, 0);
            assert_eq!(output.port_address, 1.into());
            assert_eq!(output.length.parsed_length, Some(2));
            assert_eq!(output.data.inner, vec![255, 255]);
        }
    }

    #[test]
    fn invalid_port_address() {
        // Here Port-Address is 32_768
        // Any Port-Address over 32_767 should fail
        assert!(ArtCommand::from_buffer(
            &[
                vec![65, 114, 116, 45, 78, 101, 116, 0, 0, 80, 0, 14, 0, 0,],
                32_768u16.to_le_bytes().to_vec(),
                vec![0, 2, 255, 255,]
            ]
            .concat()
        )
        .is_err());
    }
}
