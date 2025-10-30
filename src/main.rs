use srt::{
    ops::{handshake_v5, make_ack},
    packet::{Packet, PacketContent, control::ControlPacketInfo},
};

use std::{
    fs::{self, OpenOptions},
    io::Write,
    net::{SocketAddr, UdpSocket},
};

fn get_packet(socket: &UdpSocket) -> anyhow::Result<(Packet, SocketAddr)> {
    let mut buf = [0; 10000];

    let (n, addr) = socket.recv_from(&mut buf).unwrap();
    let data = &buf[..n];

    println!("\n[*] Received {n} bytes from {addr}");

    let in_packet = Packet::from_raw(data)?;

    Ok((in_packet, addr))
}

fn main() -> anyhow::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:9000")?;

    // _ = fs::remove_file("a.mpg");
    fs::write("a.mpg", []).unwrap();

    loop {
        let conn = handshake_v5(&socket)?;
        let mut ack_count = 1;

        println!("{}\nStream started\n{}", "=".repeat(14), "=".repeat(14));

        loop {
            let (packet, _) = get_packet(&socket)?;

            match packet.content {
                PacketContent::Control(ControlPacketInfo::Shutdown) => {
                    println!("{}\nShutdown\n{}", "=".repeat(14), "=".repeat(14));
                    break;
                }
                PacketContent::Control(control) => {
                    println!(" => Control: {control:?}");
                }
                PacketContent::Data(data) => {
                    // println!("{data:?}");
                    println!(
                        " => Data {{ packet_sequence_number: {:?}, position: {:?}, order: {:?}, encryption: {:?}, retransmitted: {:?}, message_number: {:?}, length: {:?} }}",
                        data.packet_sequence_number,
                        data.position,
                        data.order,
                        data.encryption,
                        data.retransmitted,
                        data.message_number,
                        data.content.len()
                    );
                    // println!(" => Payload: {:x?}", data.content);
                    let ack = make_ack(&data, ack_count)?;
                    ack_count += 1;
                    conn.send(&socket, ack)?;

                    let mut file = OpenOptions::new().append(true).open("a.mpg").unwrap();
                    file.write_all(&data.content[8..]).unwrap();
                }
            }
        }
    }
}
