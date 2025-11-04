use srt::server::Server;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_env_filter("trace").init();

    let mut srt = Server::new()?;
    srt.run()?;
    Ok(())
}
