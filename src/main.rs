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

    let system_pid = Rc::new(RefCell::new(0u16));
    let clock_pid = Rc::new(RefCell::new(0u16));

    let mut srt_server = SrtServer::new("0.0.0.0:9000")?;

    srt_server.on_connect({
        let is_ended = is_ended.clone();
        move |conn| {
            let id = conn.stream_id.clone().unwrap_or_default();
            tracing::info!("Stream started: {id:?}");
            is_ended.store(false, Ordering::Relaxed);
        }
    });

    srt_server.on_disconnect(move |conn| {
        let id = conn.stream_id.clone().unwrap_or_default();
        tracing::info!("Stream ended: {id:?}");
        is_ended.store(true, Ordering::Relaxed);
    });

    srt_server.on_data(move |_, mpeg_data| {
        for chunk in mpeg_data.chunks_exact(MPEG_PACKET_SIZE) {
            let pmt_ids = {
                let pmt = *system_pid.borrow();
                if pmt != 0 { &[pmt] } else { &[] as &[u16] }
            };

            let pack = MpegPacket::from_raw(chunk, pmt_ids).unwrap();

            match pack.payload {
                // If packet is PAS
                Some(Payload::PSI(ProgramSpecificInformation {
                    section: Section::PAS(table),
                    ..
                })) => {
                    if *system_pid.borrow() == 0 {
                        tracing::info!("Got system program id: {}", table.programs[0].program_id);
                        system_pid.replace(table.programs[0].program_id);
                    }

                    let new_segment = *timer.borrow() / segment_size;
                    let old_segment = current_segment.swap(new_segment, Ordering::Relaxed);

                    // Flush
                    if old_segment != new_segment {
                        tracing::info!("segment-write {old_segment}");

                        fs::OpenOptions::new()
                            .create(true)
                            .append(true)
                            .open(format!("_local/stream/segment_{old_segment}.mpg"))
                            .unwrap()
                            .write_all(&current_segment_data.borrow())
                            .unwrap();

                        current_segment_data.borrow_mut().clear();
                    }
                }

                // If packet is PMS
                Some(Payload::PSI(ProgramSpecificInformation {
                    section: Section::PMS(table),
                    ..
                })) => {
                    if *clock_pid.borrow() == 0 {
                        tracing::info!("Got clock program id: {}", table.pcr_pid);
                        clock_pid.replace(table.pcr_pid);
                    }
                }

                // If packet is Video (OBS) + clock
                Some(Payload::PES(pes)) if pack.header.packet_id == *clock_pid.borrow() => {
                    if let Some(pts_dts) = &pes.pes_header.as_ref().unwrap().pts_dts {
                        let seconds = pts_dts.pts() / 90_000;
                        timer.replace(seconds);
                    }
                }

                _ => {}
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

    _ = fs::remove_dir_all("_local/stream");
    fs::create_dir_all("_local/stream").unwrap();

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
