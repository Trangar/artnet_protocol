use artnet_protocol::*;
use socket2::{Domain, Protocol, Socket, Type};
use std::net::{SocketAddr, UdpSocket};

fn main() {
    let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP))
        .expect("failed to create UDP socket");
    socket.set_broadcast(true).expect("failed to set broadcast");
    socket
        .set_reuse_address(true)
        .expect("failed to enable address reuse");
    let addr = SocketAddr::from(([0, 0, 0, 0], 6454));
    socket.bind(&addr.into()).expect("failed to bind socket");

    let socket = UdpSocket::from(socket);

    loop {
        let mut buffer = [0u8; 1024];
        let (length, _addr) = socket.recv_from(&mut buffer).unwrap();
        let command = ArtCommand::from_buffer(&buffer[..length]).unwrap();

        match command {
            ArtCommand::OpTimeCode(timecode) => {
                println!("Timecode: {timecode:?} ",)
            }
            _ => {}
        }
    }
}
