use std::{
    net::{ToSocketAddrs, UdpSocket},
    str::FromStr,
};

use artnet_protocol::*;
use socket2::{Domain, Protocol, Socket, Type};

fn main() {
    let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP))
        .expect("failed to create UDP socket");
    socket.set_broadcast(true).expect("failed to set broadcast");
    socket
        .set_reuse_address(true)
        .expect("failed to enable address reuse");
    let addr = std::net::SocketAddr::new(std::net::IpAddr::from_str("0.0.0.0").unwrap(), 6454);
    socket.bind(&addr.into()).expect("failed to bind socket");

    let socket = UdpSocket::from(socket);
    let broadcast_addr = ("255.255.255.255", 6454)
        .to_socket_addrs()
        .unwrap()
        .next()
        .unwrap();
    let buff = ArtCommand::Poll(Poll::default()).write_to_buffer().unwrap();
    socket.send_to(&buff, &broadcast_addr).unwrap();

    loop {
        let mut buffer = [0u8; 1024];
        let (length, addr) = socket.recv_from(&mut buffer).unwrap();
        let command = ArtCommand::from_buffer(&buffer[..length]).unwrap();

        match command {
            ArtCommand::Poll(poll) => {
                // This will most likely be our own poll request, as this is broadcast to all devices on the network
            }
            ArtCommand::PollReply(reply) => {
                let command = ArtCommand::OpTrigger(Trigger {
                    key: TriggerKey::Show,
                    sub_key: 1,
                    ..Default::default()
                });
                let bytes = command.write_to_buffer().unwrap();
                socket.send_to(&bytes, &addr).unwrap();
            }
            _ => {}
        }
    }
}
