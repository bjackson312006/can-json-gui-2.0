//! Stuff for updating.
//! 
//! It doesn't actually do the updating. It just gives the users the file
//! 
//! Clade wrote this mostly

#![allow(dead_code)]

use std::time::Duration;

use semver::Version;
use serde::Deserialize;

const REPO: &str = "bjackson312006/can-json-gui-2.0";
const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");
const REQUEST_TIMEOUT: Duration = Duration::from_secs(10);

/// Errors that can occur while checking for an update.
#[derive(Debug, thiserror::Error)]
pub enum UpdateError {
    #[error("network request failed: {0}")]
    Network(#[from] Box<ureq::Error>),

    #[error("failed to read/parse the GitHub response: {0}")]
    Parse(#[from] std::io::Error),

    #[error("could not parse a version string as semver: {0}")]
    Semver(#[from] semver::Error),
}

/// The outcome of an update check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UpdateStatus {
    /// The running binary is the latest version (or newer, e.g. a dev build).
    UpToDate,
    /// A newer release is available.
    Available(UpdateInfo),
}

/// Details about an available update.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateInfo {
    /// The new version (e.g. `1.2.3`).
    pub version: Version,
    /// The release's git tag (e.g. `v1.2.3`).
    pub tag: String,
    /// Human-readable release notes (the release body), if any.
    pub notes: String,
    /// URL to the release's web page (fallback if no matching asset is found).
    pub release_url: String,
    /// Direct download URL for this platform's installer asset, if one matched.
    pub asset_url: Option<String>,
}

/// A GitHub release deserialized from the `/releases/latest` response.
#[derive(Debug, Deserialize)]
struct Release {
    tag_name: String,
    #[serde(default)]
    body: String,
    html_url: String,
    #[serde(default)]
    prerelease: bool,
    #[serde(default)]
    assets: Vec<Asset>,
}

#[derive(Debug, Deserialize)]
struct Asset {
    name: String,
    browser_download_url: String,
}

/// Checks GitHub for a newer release than the running binary.
///
/// This performs a blocking network request!
pub fn check_for_update() -> Result<UpdateStatus, UpdateError> {
    let release = fetch_latest_release()?;
    evaluate(&release, CURRENT_VERSION)
}

/// Fetches the newest published (non-prerelease) release from GitHub.
fn fetch_latest_release() -> Result<Release, UpdateError> {
    let url = format!("https://api.github.com/repos/{REPO}/releases/latest");

    let response = ureq::get(&url)
        .timeout(REQUEST_TIMEOUT)
        .set("User-Agent", "can-json-gui-updater")
        .set("Accept", "application/vnd.github+json")
        .set("X-GitHub-Api-Version", "2022-11-28")
        .call()
        .map_err(Box::new)?;

    let release: Release = response.into_json()?;
    Ok(release)
}

/// Compares a fetched release against the running version and decides whether
/// an update is available.
fn evaluate(release: &Release, current: &str) -> Result<UpdateStatus, UpdateError> {
    // Ignore prereleases: only stable releases should prompt an update.
    if release.prerelease {
        return Ok(UpdateStatus::UpToDate);
    }

    let current = Version::parse(current)?;
    let latest = Version::parse(release.tag_name.trim_start_matches('v'))?;

    if latest > current {
        Ok(UpdateStatus::Available(UpdateInfo {
            version: latest,
            tag: release.tag_name.clone(),
            notes: release.body.clone(),
            release_url: release.html_url.clone(),
            asset_url: pick_asset(&release.assets).map(|a| a.browser_download_url.clone()),
        }))
    } else {
        Ok(UpdateStatus::UpToDate)
    }
}

/// Selects the release asset that matches the current platform's installer
/// format, or `None` if nothing suitable is present.
fn pick_asset(assets: &[Asset]) -> Option<&Asset> {
    // The installer extension we expect for this platform.
    let wanted_ext = if cfg!(target_os = "windows") {
        ".msi"
    } else if cfg!(target_os = "macos") {
        ".dmg"
    } else {
        ".deb"
    };

    assets
        .iter()
        .find(|a| a.name.to_ascii_lowercase().ends_with(wanted_ext))
}