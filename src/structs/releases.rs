use crate::{
    consts::{GITHUB_REPO_NAME, GITHUB_REPO_OWNER},
    structs::Release,
};
use color_eyre::eyre::{Context, Report, Result};
use reqwest::{Client, Response};
use semver::Version;

#[derive(Debug)]
pub struct Releases(pub Vec<Release>);
impl Releases {
    /// Guarantee that every release has a valid semver version. If at least one doesn't, an error
    /// will be returned.
    /// Almost every other function implemented on this struct assumes this function has been
    /// called at least once.
    pub fn guarantee_versions(&self) -> Result<()> {
        for release in &self.0 {
            release.try_as_version().wrap_err_with(|| {
                format!("Tried parsing the version for the release {:#?}.", release)
            })?;
        }
        // Ok.
        Ok(())
    }
    /// Sorts the releases by their version, so the newest release is the first.
    pub fn sort_unstable_by_version(&mut self) -> Result<()> {
        self.0.sort_unstable_by_key(|release: &Release| -> Version {
            release.try_as_version().unwrap()
        });
        // Ok.
        Ok(())
    }
    /// Finds the release that has the given version.
    pub fn find_with_version(&self, version: &Version) -> Option<&Release> {
        self.0
            .iter()
            .find(|release: &&Release| -> bool { release.try_as_version().unwrap() == *version })
    }
    /// Returns a new Releases that contains all releases newer than the passed version.
    pub fn newer_than<T>(&self, version: T) -> Result<Self>
    where
        T: TryInto<Version>,
        T::Error: Into<Report>,
    {
        let version: Version = version.try_into().map_err(Into::into)?;
        Ok(Self(
            self.0
                .iter()
                .take_while(|release: &&Release| -> bool {
                    release.try_as_version().unwrap() != version
                })
                .cloned()
                .collect(),
        ))
    }
    /// Returns a new Releases that contains all releases older than the passed version.
    pub fn older_than<T>(&self, version: T) -> Result<Self>
    where
        T: TryInto<Version>,
        T::Error: Into<Report>,
    {
        let version: Version = version.try_into().map_err(Into::into)?;
        Ok(Self(
            self.0
                .iter()
                .skip_while(|release: &&Release| -> bool {
                    release.try_as_version().unwrap() == version
                })
                .cloned()
                .collect(),
        ))
    }
    /// Returns a vector of the versions.
    pub fn as_versions(&self) -> Vec<Version> {
        self.0
            .iter()
            .map(|release: &Release| -> Version { release.try_as_version().unwrap() })
            .collect()
    }
    /// Fetches the releases from the github repository.
    /// Automatically calls self.guarantee_versions().
    pub async fn fetch() -> Result<Self> {
        let client: Client = Client::new();
        let mut all_releases: Vec<Release> = Vec::new();
        let mut page: usize = 1;
        loop {
            let api_url: String = format!(
                "https://api.github.com/repos/{}/{}/releases?per_page=100&page={}",
                GITHUB_REPO_OWNER, GITHUB_REPO_NAME, page
            );
            let response: Response = client
                .get(api_url)
                .header("User-Agent", "spalst_updater")
                .send()
                .await?;
            let releases: Vec<Release> = response.json().await?;
            if releases.is_empty() {
                break;
            };
            all_releases.extend(releases);
            page += 1;
        }
        // Wrap all_releases in the Self type.
        let releases: Self = Self(all_releases);
        // Guarantee that the versions self has are good semver wise.
        releases.guarantee_versions()?;
        // Ok.
        Ok(releases)
    }
}
