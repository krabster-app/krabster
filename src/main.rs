mod configuration;
mod downloader;

use anyhow::Result;
use axum::{extract::Path, routing::get, Json, Router};
use downloader::base::Download;
use std::net;
use std::path;

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new().route("/yt/:id", get(get_endpoint));

    axum::Server::bind(&net::SocketAddr::from(([127, 0, 0, 1], 3000)))
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn get_endpoint(Path(id): Path<String>) -> Json<String> {
    let url = format!("https://www.youtube.com/watch?v={id}");

    let path = format!("/tmp/{id}");
    let path = path::Path::new(&path);

    downloader::youtube::YoutubeDownload::new(String::from(url))
        .download(path.to_path_buf())
        .await
        .unwrap();

    Json(id)
}
