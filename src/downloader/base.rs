use anyhow::Result;
use std::path::PathBuf;

#[async_trait::async_trait]
pub trait Download {
    async fn download(&self, path: PathBuf) -> Result<()>;
}
