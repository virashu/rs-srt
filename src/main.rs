use rs_srt::packet::control::{
    ControlInformation,
    handshake::{Handshake, HandshakeEncryption, HandshakeType},
};
use std::net::UdpSocket;

use rs_srt::packet::{Packet, PacketContent};

fn main() -> anyhow::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:9000")?;

    let mut buf = [0; 1024];

    let (n, addr) = socket.recv_from(&mut buf)?;
    let data = &buf[..n];
    println!("[*] Received {n} bytes");

    let in_packet = Packet::from_raw(data)?;
    println!(" IN\n{in_packet:#?}");

    let PacketContent::Control(ControlInformation::Handshake(handshake)) = in_packet.content else {
        panic!("Failed to unwrap handshake");
    };

    println!("\n{}\n", "=".repeat(40));

    let out_packet_v4 = Packet {
        timestamp: in_packet.timestamp + 1,
        dest_socket_id: handshake.srt_socket_id,
        // dest_socket_id: 0,
        content: PacketContent::Control(ControlInformation::Handshake(Handshake {
            srt_socket_id: 42,
            syn_cookie: 42,
            ..handshake
        })),
    };
    println!(
        " OUT ({})\n\n{out_packet_v4:#?}",
        out_packet_v4.to_raw().len()
    );

    println!("\n{}\n", "=".repeat(40));

    socket.send_to(&out_packet_v4.to_raw(), addr)?;

    let (n, _) = socket.recv_from(&mut buf)?;
    let data = &buf[..n];
    let in_packet = Packet::from_raw(data)?;
    println!(" IN\n{in_packet:#?}");

    println!("Done");

    Ok(())
}
