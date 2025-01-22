//! Listen for a [FlexRadio Discovery packet] on port 4992 and print the contents
//! of the data payload.
//!
//! [FlexRadio Discovery packet]: https://github.com/flexradio/smartsdr-api-docs/wiki/Discovery-protocol

use std::net::UdpSocket;

use vrt::VrtPacket;

fn main() -> std::io::Result<()> {
    // Bind the UDP socket to port 4992 on all available interfaces
    let socket = UdpSocket::bind("0.0.0.0:4992")?;
    println!("Listening on {}", socket.local_addr()?);

    // Buffer to hold incoming data
    let mut buf = [0u8; 1024];

    loop {
        // Receive data from the socket
        let (len, src) = socket.recv_from(&mut buf)?;
        println!("Received {} bytes from {}", len, src);

        // Send a reply back to the source
        match VrtPacket::parse(&buf[..len]) {
            Ok((_rest, packet)) => {
                println!(
                    "Received packet: {:?}",
                    String::from_utf8_lossy(packet.payload)
                );
            }
            Err(e) => {
                println!("Failed to parse packet: {:?}", e);
            }
        }
    }
}
