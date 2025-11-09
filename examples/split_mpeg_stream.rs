use std::{
    cell::RefCell,
    fs,
    io::Write,
    rc::Rc,
    sync::{Arc, Mutex},
};

use mpeg::{
    psi::packet::{ProgramSpecificInformation, Section},
    transport::packet::{Payload, TransportPacket as MpegPacket},
};
use srt::{connection::Connection, server::Server as SrtServer};

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_env_filter("info").init();

    _ = fs::remove_dir_all("_local/stream");
    fs::create_dir_all("_local/stream").unwrap();

    let mut srt_server = SrtServer::new("0.0.0.0:9000")?;

    srt_server.on_connect(|conn| {
        tracing::info!(
            "Client connected: {:?}",
            conn.stream_id.clone().unwrap_or_default()
        );
    });

    srt_server.on_disconnect(|conn| {
        tracing::info!(
            "Client disconnected: {:?}",
            conn.stream_id.clone().unwrap_or_default()
        );
    });

    let segment_s = Rc::new(RefCell::new(0u64));
    let segment = Rc::new(RefCell::new(0u64));
    let pids_pmt = Arc::new(Mutex::new(Vec::new()));

    let on_data = move |_: &Connection, mpeg_data: &[u8]| {
        for chunk in mpeg_data.chunks_exact(188) {
            let pack = MpegPacket::from_raw(chunk, &pids_pmt.lock().unwrap()).unwrap();

            // If packet is PAS
            if matches!(
                pack.payload,
                Some(Payload::PSI(ProgramSpecificInformation {
                    section: Section::PAS(_),
                    ..
                }))
            ) {
                segment.replace(*segment_s.borrow());
            }

            if pack.header.pid == 0x100
                && let Some(Payload::PES(pes)) = pack.payload
            {
                let seconds = pes.pes_header.unwrap().pts_dts.unwrap().pts() / 90_000;
                let segment_n = seconds / 2;
                segment_s.replace(segment_n);
            }

            let mut file = fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(format!("_local/stream/segment_{}.mpg", segment.borrow()))
                .unwrap();

            file.write_all(chunk).unwrap();
        }
    };

    srt_server.on_data(on_data);

    tracing::info!("Starting SRT");
    srt_server.run()?;

    Ok(())
}
