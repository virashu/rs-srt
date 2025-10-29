use rs_srt::packet::{
    Packet, PacketContent,
    control::{ControlInformation, handshake::Handshake},
};
use std::net::UdpSocket;

fn handshake_v4(socket: &UdpSocket) -> anyhow::Result<()> {
    let mut buf = [0; 80];

    println!("Waiting for a handshake...");

    //
    // Induction
    //

    let (n, addr) = socket.recv_from(&mut buf)?;
    let data = &buf[..n];

    println!("Connection: {addr}");

    let in_packet = Packet::from_raw(data)?;

    let PacketContent::Control(ControlInformation::Handshake(handshake)) = in_packet.content else {
        return Err(anyhow::anyhow!("Failed to unwrap handshake"));
    };

    let out_packet_v4 = Packet {
        timestamp: in_packet.timestamp + 1,
        dest_socket_id: handshake.srt_socket_id,
        content: PacketContent::Control(ControlInformation::Handshake(Handshake {
            srt_socket_id: 42,
            syn_cookie: 42,
            ..handshake
        })),
    };

    socket.send_to(&out_packet_v4.to_raw(), addr)?;

    println!("Completed Induction");

    //
    // Conclusion
    //

    let (n, addr) = socket.recv_from(&mut buf)?;
    let data = &buf[..n];

    let in_packet = Packet::from_raw(data)?;

    let PacketContent::Control(ControlInformation::Handshake(handshake)) = in_packet.content else {
        return Err(anyhow::anyhow!("Failed to unwrap handshake"));
    };

    let out_packet_v4 = Packet {
        timestamp: in_packet.timestamp + 1,
        dest_socket_id: handshake.srt_socket_id,
        content: PacketContent::Control(ControlInformation::Handshake(Handshake { ..handshake })),
    };

    socket.send_to(&out_packet_v4.to_raw(), addr)?;

    println!("Completed Conclusion");

    println!("Done!");

    Ok(())
}

fn handshake_v5(socket: &UdpSocket) -> anyhow::Result<()> {
    const MAGIC_CODE: u16 = 0x4A17;

    let mut buf = [0; 80];

    println!("Waiting for a handshake...");

    //
    // Induction
    //

    let (n, addr) = socket.recv_from(&mut buf)?;
    let data = &buf[..n];

    println!("Connection: {addr}");

    let in_packet = Packet::from_raw(data)?;

    let PacketContent::Control(ControlInformation::Handshake(handshake)) = in_packet.content else {
        return Err(anyhow::anyhow!("Failed to unwrap handshake"));
    };

    let out_packet_v5 = Packet {
        timestamp: in_packet.timestamp + 1,
        dest_socket_id: handshake.srt_socket_id,
        content: PacketContent::Control(ControlInformation::Handshake(Handshake {
            version: 5,
            extension_field: MAGIC_CODE,
            srt_socket_id: 42,
            syn_cookie: 42,
            ..handshake
        })),
    };

    socket.send_to(&out_packet_v5.to_raw(), addr)?;

    println!("Completed Induction");

    //
    // Conclusion
    //

    let (n, addr) = socket.recv_from(&mut buf)?;
    let data = &buf[..n];

    let in_packet = Packet::from_raw(data)?;

    let PacketContent::Control(ControlInformation::Handshake(handshake)) = in_packet.content else {
        return Err(anyhow::anyhow!("Failed to unwrap handshake"));
    };

    let out_packet_v5 = Packet {
        timestamp: in_packet.timestamp + 1,
        dest_socket_id: handshake.srt_socket_id,
        content: PacketContent::Control(ControlInformation::Handshake(Handshake { ..handshake })),
    };

    socket.send_to(&out_packet_v5.to_raw(), addr)?;

    println!("Completed Conclusion");

    println!("Done!");

    Ok(())
}

fn handle_packet(socket: &UdpSocket) -> anyhow::Result<()> {
    let mut buf = [0; 10000];

    let (n, addr) = socket.recv_from(&mut buf).unwrap();
    let data = &buf[..n];

    println!("[*] Received {n} bytes");

    let in_packet = Packet::from_raw(data)?;

    // thread::sleep(Duration::from_secs(1));

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:9000")?;

    handshake_v5(&socket)?;

    println!("{}\nStream started\n{}", "=".repeat(14), "=".repeat(14));

    loop {
        handle_packet(&socket)?;
    }
}
