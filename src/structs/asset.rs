use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Asset {
    name: String,
    browser_download_url: String,
    size: u64,
}
