use artnet_protocol::*;
use std::net::{ToSocketAddrs, UdpSocket};

fn main() {
    let socket = UdpSocket::bind(("0.0.0.0", 6454)).unwrap();
    let broadcast_addr = ("255.255.255.255", 6454)
        .to_socket_addrs()
        .unwrap()
        .next()
        .unwrap();
    socket.set_broadcast(true).unwrap();
    let buff = ArtCommand::Poll(Poll::default()).write_to_buffer().unwrap();
    socket.send_to(&buff, &broadcast_addr).unwrap();

    loop {
        let mut buffer = [0u8; 1024];
        let (length, addr) = socket.recv_from(&mut buffer).unwrap();
        let command = ArtCommand::from_buffer(&buffer[..length]).unwrap();

        println!("Received {:?}", command);
        match command {
            ArtCommand::Poll(poll) => {
                // This will most likely be our own poll request, as this is broadcast to all devices on the network
            }
            ArtCommand::PollReply(reply) => {
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
