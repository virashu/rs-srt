use srt::server::Server;
use tracing::Level;

use std::{fs, io::Write, time::SystemTime};

fn run_hls() -> anyhow::Result<()> {
    tracing::info!("Starting HLS");
    hls::run();

    Ok(())
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let mut srt_server = Server::new()?;

    srt_server.on_connect(&|conn| {
        tracing::info!(
            "Client connected: {:?}",
            conn.stream_id.clone().unwrap_or_default()
        );
    });

    srt_server.on_disconnect(&|conn| {
        tracing::info!(
            "Client disconnected: {:?}",
            conn.stream_id.clone().unwrap_or_default()
        );
    });

    srt_server.on_data(&|conn, mpeg_packet| {
        let id = conn.stream_id.clone().unwrap_or_default();

        println!("{mpeg_packet:?}");
    });

    tracing::info!("Starting SRT");
    srt_server.run()?;

    Ok(())
}
