use std::{net::UdpSocket, time::SystemTime};

use crate::{
    connection::Connection,
    packet::{
        Packet, PacketContent,
        control::{ControlPacketInfo, ack::Ack, handshake::Handshake},
        data::DataPacketInfo,
    },
};

pub fn handshake_v4(socket: &UdpSocket) -> anyhow::Result<()> {
    let mut buf = [0; 80];

    println!("Waiting for a handshake...");

    //
    // Induction
    //

    let (n, addr) = socket.recv_from(&mut buf)?;
    let data = &buf[..n];

    println!("Connection: {addr}");

    let in_packet = Packet::from_raw(data)?;

    let PacketContent::Control(ControlPacketInfo::Handshake(handshake)) = in_packet.content else {
        return Err(anyhow::anyhow!("Failed to unwrap handshake"));
    };

    let out_packet_v4 = Packet {
        timestamp: in_packet.timestamp + 1,
        dest_socket_id: handshake.srt_socket_id,
        content: PacketContent::Control(ControlPacketInfo::Handshake(Handshake {
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

    let PacketContent::Control(ControlPacketInfo::Handshake(handshake)) = in_packet.content else {
        return Err(anyhow::anyhow!("Failed to unwrap handshake"));
    };

    let out_packet_v4 = Packet {
        timestamp: in_packet.timestamp + 1,
        dest_socket_id: handshake.srt_socket_id,
        content: PacketContent::Control(ControlPacketInfo::Handshake(Handshake { ..handshake })),
    };

    socket.send_to(&out_packet_v4.to_raw(), addr)?;

    println!("Completed Conclusion");

    println!("Done!");

    Ok(())
}

pub fn handshake_v5(socket: &UdpSocket) -> anyhow::Result<Connection> {
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

    let PacketContent::Control(ControlPacketInfo::Handshake(handshake)) = in_packet.content else {
        return Err(anyhow::anyhow!("Failed to unwrap handshake"));
    };

    let out_packet_v5 = Packet {
        timestamp: in_packet.timestamp + 1,
        dest_socket_id: handshake.srt_socket_id,
        content: PacketContent::Control(ControlPacketInfo::Handshake(Handshake {
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

    let PacketContent::Control(ControlPacketInfo::Handshake(handshake)) = in_packet.content else {
        return Err(anyhow::anyhow!("Failed to unwrap handshake"));
    };

    let out_packet_v5 = Packet {
        timestamp: in_packet.timestamp + 1,
        dest_socket_id: handshake.srt_socket_id,
        content: PacketContent::Control(ControlPacketInfo::Handshake(Handshake { ..handshake })),
    };

    socket.send_to(&out_packet_v5.to_raw(), addr)?;

    println!("Completed Conclusion");

    let est = SystemTime::now();

    println!("Done!");

    Ok(Connection {
        est,
        addr,
        peer_srt_socket_id: handshake.srt_socket_id,
    })
}

pub fn make_ack(data: &DataPacketInfo, count: u32) -> anyhow::Result<PacketContent> {
    Ok(PacketContent::Control(ControlPacketInfo::Ack(Ack {
        ack_number: count,
        last_ackd_packet_sequence_number: data.packet_sequence_number + 1,
        rtt: 0,
        rtt_variance: 0,
        available_buffer_size: 1,
        packets_receiving_rate: 4,
        estimated_link_capacity: 4,
        receiving_rate: 4096,
    })))
}
