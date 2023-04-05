use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ProvidersConfiguration {
    youtube: Option<YoutubeConfiguration>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct YoutubeConfiguration {}
