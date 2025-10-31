use srt::server::Server;
use tracing::Level;

use std::{
    fs::{self, OpenOptions},
    io::Write,
};

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let mut server = Server::new()?;

    server.on_connect(&|id| {
        fs::write(format!("_local/stream_{id}.mpg"), []).unwrap();
    });

    server.on_data(&|id: &str, mpeg_packet: &[u8]| {
        let mut file = OpenOptions::new()
            .append(true)
            .open(format!("_local/stream_{id}.mpg"))
            .unwrap();

        file.write_all(mpeg_packet).unwrap();
    });

    server.run()?;

    Ok(())
}
