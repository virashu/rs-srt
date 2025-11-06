// pub fn handshake_v4(socket: &UdpSocket) -> anyhow::Result<Connection> {
//     let mut buf = [0; 80];

//     //
//     // Induction
//     //

//     let (n, addr) = socket.recv_from(&mut buf)?;
//     let data = &buf[..n];

//     let in_packet = Packet::from_raw(data)?;
//     let PacketContent::Control(ControlPacketInfo::Handshake(handshake)) = in_packet.content else {
//         return Err(anyhow::anyhow!("Failed to unwrap handshake"));
//     };

//     let stream_id = handshake
//         .stream_id_extension
//         .as_ref()
//         .map(|x| x.stream_id.clone());

//     let out_packet_v4 = Packet {
//         timestamp: in_packet.timestamp + 1,
//         dest_socket_id: handshake.srt_socket_id,
//         content: PacketContent::Control(ControlPacketInfo::Handshake(Handshake {
//             srt_socket_id: 42,
//             syn_cookie: 42,
//             ..handshake
//         })),
//     };
//     socket.send_to(&out_packet_v4.to_raw(), addr)?;

//     //
//     // Conclusion
//     //

//     let (n, addr) = socket.recv_from(&mut buf)?;
//     let data = &buf[..n];

//     let in_packet = Packet::from_raw(data)?;
//     let PacketContent::Control(ControlPacketInfo::Handshake(handshake)) = in_packet.content else {
//         return Err(anyhow::anyhow!("Failed to unwrap handshake"));
//     };

//     let out_packet_v4 = Packet {
//         timestamp: in_packet.timestamp + 1,
//         dest_socket_id: handshake.srt_socket_id,
//         content: PacketContent::Control(ControlPacketInfo::Handshake(Handshake {
//             version: 4,
//             ..handshake
//         })),
//     };
//     socket.send_to(&out_packet_v4.to_raw(), addr)?;
// }
