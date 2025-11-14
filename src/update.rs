use crate::{
    consts::{GITHUB_REPO_NAME, GITHUB_REPO_OWNER},
    structs::Version,
};
use color_eyre::eyre::Result;
use serde::Deserialize;

#[derive(Deserialize)]
struct VersionInfo {
    pub latest: String,
    pub bad_versions: Vec<(String, String)>,
}

#[derive(Debug, Deserialize)]
struct Release {
    tag_name: String,
    name: Option<String>,
    body: Option<String>,
    html_url: String,
    //todo
}

pub fn updater() -> Result<()> {
    // Get the current version
    let current_version: Version = env!("CARGO_PKG_VERSION").parse()?;
    // Get the VersionInfo
    let version_info: VersionInfo = fetch_version_info()?;
    // Get the latest version
    let latest_version: Version = version_info.latest.parse()?;
    //let latest_version: Version = get_latest_version()?.parse()?;
    // Get the bad versions
    let mut bad_versions: Vec<(Version, Version)> = Vec::new();
    for (bad_version, fixed_version) in version_info.bad_versions {
        bad_versions.push((bad_version.parse()?, fixed_version.parse()?));
    }
    // If the current version matches the latest, exit
    if current_version == latest_version {
        return Ok(());
    };
    // If the current version is bigger than the release version, notify the user and exit
    if current_version > latest_version {
        eprintln!("The current version is a higher version than the released version.");
        return Ok(());
    };
    // The program is an outdated version and needs an update.
    // If the current version is the first element in any of the tuples in bad_versions, force
    // update.
    for (bad_ver, fixed_ver) in bad_versions {
        if bad_ver != current_version {
            continue;
        };
        // The current version is bad. Forced update.
        eprintln!(
            "Current version ({}) has a critical bug. Updating to the first version that fixes it ({})...",
            current_version, fixed_ver
        );
        update_to_ver(fixed_ver)?;
    }
    todo!()
}

fn fetch_version_info() -> Result<VersionInfo> {
    let url: &str = "https://raw.githubusercontent.com/Stefan-Rankovic/Spalst/master/versions.json";
    let response: String = reqwest::blocking::get(url)?.text()?;
    let info: VersionInfo = serde_json::from_str(&response)?;
    Ok(info)
}

fn update_to_ver(ver: Version) -> Result<()> {
    // Define the URL
    let url: String = format!(
        "https://api.github.com/repos/{}/{}/releases/tags/{}",
        GITHUB_REPO_OWNER, GITHUB_REPO_NAME, ver,
    );
    let release: Release = reqwest::blocking::Client::new()
        .get(&url)
        .header("User-Agent", "reqwest")
        .send()?
        .error_for_status()?
        .json()?;

    dbg!(&release);

    todo!()
}
