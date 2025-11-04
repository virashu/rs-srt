use std::{
    fmt::Write,
    fs,
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicU64, Ordering},
    },
};

use axum::{
    Router,
    body::Body,
    extract::{Path, State},
    http::{
        Response, StatusCode,
        header::{ACCESS_CONTROL_ALLOW_ORIGIN, CONTENT_TYPE},
    },
    response::IntoResponse,
    routing::get,
};

const PLAYLIST_HEADER_EVENT: &str = "#EXTM3U\n#EXT-X-VERSION:3\n#EXT-X-PLAYLIST-TYPE:EVENT\n";

fn read_playlist(segment_size: u64, _current_segment: u64, is_ended: bool) -> String {
    let mut res = String::from(PLAYLIST_HEADER_EVENT);

    let mut ents: Vec<_> = fs::read_dir("_local")
        .unwrap()
        .filter_map(Result::ok)
        .collect();

    let count = ents.len();
    let skip = count.saturating_sub(5);

    writeln!(res, "#EXT-X-TARGETDURATION:{segment_size}").unwrap();
    writeln!(res, "#EXT-X-MEDIA-SEQUENCE:{skip}").unwrap();

    ents.sort_by_key(|x| (x.file_name().len(), x.file_name()));

    for ent in ents.iter().skip(skip) {
        writeln!(res, "#EXTINF:{segment_size}.000,").unwrap();
        writeln!(
            res,
            "/api/hls/segment/{}",
            ent.file_name().to_str().unwrap()
        )
        .unwrap();
    }

    if is_ended {
        res += "#EXT-X-ENDLIST";
    }

    res
}

async fn get_playlist(State(state): State<AppState>) -> impl IntoResponse {
    let segment_size = state.segment_size;
    let current_segment = state.current_segment.load(Ordering::Relaxed);
    let is_ended = state.is_ended.load(Ordering::Relaxed);

    Response::builder()
        .header(CONTENT_TYPE, "application/vnd.apple.mpegurl")
        .header(ACCESS_CONTROL_ALLOW_ORIGIN, "*")
        .body(read_playlist(segment_size, current_segment, is_ended))
        .unwrap()
}

async fn get_segment(Path(segment): Path<String>) -> Result<impl IntoResponse, StatusCode> {
    let file_res = fs::read(format!("_local/{segment}"));

    if let Ok(file) = file_res {
        Ok(Response::builder()
            .header(CONTENT_TYPE, "application/octet-stream")
            .header(ACCESS_CONTROL_ALLOW_ORIGIN, "*")
            .body(Body::from(file))
            .unwrap())
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

#[derive(Clone, Debug)]
struct AppState {
    pub segment_size: u64,
    pub current_segment: Arc<AtomicU64>,
    pub is_ended: Arc<AtomicBool>,
}

pub fn run(
    segment_size: u64,
    current_segment: Arc<AtomicU64>,
    is_ended: Arc<AtomicBool>,
) -> anyhow::Result<()> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;

    let app = Router::new()
        .route("/api/hls/stream.m3u8", get(get_playlist))
        .route("/api/hls/segment/{segment}", get(get_segment))
        .with_state(AppState {
            segment_size,
            current_segment,
            is_ended,
        });

    let listener = rt.block_on(tokio::net::TcpListener::bind("0.0.0.0:3000"))?;
    rt.block_on(async { axum::serve(listener, app).await })?;

    Ok(())
}
