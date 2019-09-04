//! Contains the [ArtCommand](struct.ArtCommand.html) enum which holds the entire ArtNet protocol v4, as per [https://artisticlicence.com/WebSiteMaster/User%20Guides/art-net.pdf](https://artisticlicence.com/WebSiteMaster/User%20Guides/art-net.pdf)
//!
//! ```rust
//! let socket = UdpSocket::bind(("0.0.0.0", 6454))?;
//! let broadcast_addr = ("255.255.255.255", 6454).to_socket_addrs().unwrap().next().unwrap();
//! socket.set_broadcast(true).unwrap();
//! let buff = ArtCommand::Poll(Poll::default()).into_buffer().unwrap();
//! socket.send_to(&buff, &broadcast_addr).unwrap();
//!
//! loop {
//!     let mut buffer = [0u8; 1024];
//!     let (length, addr) = socket.recv_from(&mut buffer).unwrap();
//!     let command = ArtCommand::from_buffer(&buffer[..length]).unwrap();
//!     
//!     println!("Received {:?}", command);
//!     match command {
//!         ArtCommand::Poll(poll) => {
//!             // This will most likely be our own poll request, as this is broadcast to all devices on the network
//!         },
//!         ArtCommand::PollReply(reply) => {
//!             // This is an ArtNet node on the network. We can send commands to it like this:
//!             let command = ArtCommand::Output(Output {
//!                 length: 5, // must match your data.len()
//!                 data: vec![1, 2, 3, 4, 5], // The data we're sending to the node
//!                 ..Output::default()
//!             });
//!             let bytes = command.into_bytes().unwrap();
//!             socket.send_to(&bytes, &addr).unwrap();
//!         },
//!         _ => {}
//!     }
//! }
//! ```
#![deny(missing_docs)]

/// Re-export of the bitflags crate that this library uses
#[macro_use]
pub extern crate bitflags;
/// Re-export of the byteorder crate that this library uses
pub extern crate byteorder;

#[macro_use]
mod macros;
mod command;
mod convert;
mod enums;
mod error;

pub use crate::command::*;
pub use crate::enums::ArtTalkToMe;
pub use crate::error::*;
