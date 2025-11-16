/// SPDX-License-Identifier: GPL-3.0-only
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Asset {
    pub name: String,
    pub browser_download_url: String,
    size: u64,
}
