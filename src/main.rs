use std::{
    cell::RefCell,
    fs,
    io::Write,
    rc::Rc,
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicU64, Ordering},
    },
    thread,
};

use mpeg::{
    constants::PACKET_SIZE as MPEG_PACKET_SIZE,
    psi::packet::{ProgramSpecificInformation, Section},
    transport::packet::{Payload, TransportPacket as MpegPacket},
};
use srt::server::Server as SrtServer;

fn run_srt(
    segment_size: u64,
    current_segment: Arc<AtomicU64>,
    is_ended: Arc<AtomicBool>,
) -> anyhow::Result<()> {
    let timer = Rc::new(RefCell::new(0u64));
    let current_segment_data = Rc::new(RefCell::new(Vec::<u8>::new()));

    let mut srt_server = SrtServer::new()?;

    srt_server.on_connect(|conn| {
        let id = conn.stream_id.clone().unwrap_or_default();
        tracing::info!("Stream started: {id:?}");
    });

    srt_server.on_disconnect(move |conn| {
        let id = conn.stream_id.clone().unwrap_or_default();
        tracing::info!("Stream ended: {id:?}");
        is_ended.store(true, Ordering::Relaxed);
    });

    srt_server.on_data(move |_, mpeg_data| {
        for chunk in mpeg_data.chunks_exact(MPEG_PACKET_SIZE) {
            let pack = MpegPacket::from_raw(chunk, &[]).unwrap();

            // If packet is PAS
            if matches!(
                pack.payload,
                Some(Payload::PSI(ProgramSpecificInformation {
                    section: Section::PAS(_),
                    ..
                }))
            ) {
                let new_segment = *timer.borrow() / segment_size;
                let old_segment = current_segment.swap(new_segment, Ordering::Relaxed);

                // Flush
                if old_segment != new_segment {
                    tracing::info!("segment-write {old_segment}");

                    fs::OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(format!("_local/segment_{old_segment}.mpg"))
                        .unwrap()
                        .write_all(&current_segment_data.borrow())
                        .unwrap();

                    current_segment_data.borrow_mut().clear();
                }
            }

            // If packet is Video (OBS)
            if pack.header.packet_id == 0x100
                && let Some(Payload::PES(pes)) = pack.payload
            {
                let seconds = pes.pes_header.unwrap().pts_dts.unwrap().pts() / 90_000;
                timer.replace(seconds);
            }

            current_segment_data.borrow_mut().extend(chunk);
        }
    });

    srt_server.run()?;

    Ok(())
}

fn main() -> anyhow::Result<()> {
    const SECONDS_PER_SEGMENT: u64 = 2;

    tracing_subscriber::fmt().with_env_filter("info").init();

    _ = fs::remove_dir_all("_local");
    fs::create_dir_all("_local").unwrap();

    // State
    let current_segment = Arc::new(AtomicU64::new(0));
    let is_ended = Arc::new(AtomicBool::new(false));

    tracing::info!("Starting SRT");
    thread::spawn({
        let current_segment = current_segment.clone();
        let is_ended = is_ended.clone();

        || run_srt(SECONDS_PER_SEGMENT, current_segment, is_ended).unwrap()
    });

    tracing::info!("Starting HLS");
    hls::run(SECONDS_PER_SEGMENT, current_segment, is_ended)?;

    Ok(())
}
