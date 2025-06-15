use artnet_protocol::*;
use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind(("0.0.0.0", 6454)).unwrap();

    loop {
        let mut buffer = [0u8; 1024];
        let (length, _addr) = socket.recv_from(&mut buffer).unwrap();
        let command = ArtCommand::from_buffer(&buffer[..length]).unwrap();

        // println!("Received {:?}", command);
        match command {
            ArtCommand::OpTimeCode(timecode) => {
                println!("Timecode: {timecode:?} ",)
            }
            _ => {}
        }
    }
}
