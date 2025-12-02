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
    let broadcast_addr = SocketAddr::from(([255, 255, 255, 255], 6454));

    let buff = ArtCommand::Poll(Poll::default()).write_to_buffer().unwrap();
    socket.send_to(&buff, &broadcast_addr).unwrap();

    loop {
        let mut buffer = [0u8; 1024];
        let (length, addr) = socket.recv_from(&mut buffer).unwrap();
        let command = ArtCommand::from_buffer(&buffer[..length]).unwrap();

        println!("Received {:?}", command);
        match command {
            ArtCommand::Poll(_poll) => {
                // This will most likely be our own poll request, as this is broadcast to all devices on the network
            }
            ArtCommand::PollReply(_reply) => {
                // This is an ArtNet node on the network. We can send commands to it like this:
                let command = ArtCommand::Output(Output {
                    data: vec![1, 2, 3, 4, 5].into(), // The data we're sending to the node
                    ..Output::default()
                });
                let bytes = command.write_to_buffer().unwrap();
                socket.send_to(&bytes, &addr).unwrap();
            }
            _ => {}
        }
    }
}
