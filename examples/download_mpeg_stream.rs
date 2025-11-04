use std::{fs, io::Write};

use srt::server::Server;
use tracing::Level;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let mut srt_server = Server::new()?;

    srt_server.on_connect(|conn| {
        let id = conn.stream_id.clone().unwrap_or_default();
        tracing::info!("Client connected: {id:?}");
        fs::write(format!("_local/stream_{id}.mpg"), []).unwrap();
    });

    srt_server.on_disconnect(|conn| {
        let id = conn.stream_id.clone().unwrap_or_default();
        tracing::info!("Client disconnected: {id:?}");
    });

    srt_server.on_data(|conn, mpeg_packet| {
        let id = conn.stream_id.clone().unwrap_or_default();

        let mut file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(format!("_local/stream_{id}.mpg"))
            .unwrap();

        file.write_all(mpeg_packet).unwrap();
    });

    tracing::info!("Starting SRT");
    srt_server.run()
}
