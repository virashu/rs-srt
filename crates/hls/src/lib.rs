use axum::{
    Router,
    http::{Response, header::CONTENT_TYPE},
    response::IntoResponse,
    routing::get,
};

fn mock_playlist() -> String {
    String::from(
        "
        #EXTM3U
        #EXT-X-VERSION:3
        #EXT-X-PLAYLIST-TYPE:EVENT
        #EXT-X-TARGETDURATION:10
        #EXT-X-VERSION:4
        #EXT-X-MEDIA-SEQUENCE:0

        #EXTINF:2.00,
        ",
    )
}

async fn get_playlist() -> impl IntoResponse {
    println!("REQ");
    Response::builder()
        .header(CONTENT_TYPE, "application/vnd.apple.mpegurl")
        .body(mock_playlist())
        .unwrap();
}

fn router() -> Router {
    Router::new().route("/api/hls/stream.m3u8", get(get_playlist))
}

pub fn run() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    let app = router();

    rt.block_on(async {
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        axum::serve(listener, app).await.unwrap();
    });
}
