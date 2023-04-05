use super::base::Download;
use anyhow::Result;
use rustube::url::Url;
use std::{path::PathBuf, str::FromStr};

pub struct YoutubeDownload {
    url: String,
}

#[async_trait::async_trait]
impl Download for YoutubeDownload {
    async fn download(&self, save_to: PathBuf) -> Result<()> {
        let url = Url::from_str(&self.url).unwrap();

        let video = rustube::Video::from_url(&url).await?;

        std::fs::File::create(&save_to)?;

        video
            .best_audio()
            .unwrap()
            .download_to(save_to.as_path())
            .await?;

        Ok(())
    }
}

impl YoutubeDownload {
    pub fn new(url: String) -> Self {
        YoutubeDownload { url }
    }
}
