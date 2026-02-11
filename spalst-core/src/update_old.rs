//! SPDX-License-Identifier: GPL-3.0-only

use crate::{
    consts::{GITHUB_REPO_NAME, GITHUB_REPO_OWNER},
    enums::GetInputMode,
    structs::{Release, Releases, UnsafeVersion, UnsafeVersionReason, UnsafeVersions},
    utils::{epr_dbg, epr_wrn},
};
use color_eyre::eyre::{OptionExt, Result, bail, eyre};
use log::{debug, warn};
use semver::Version;
use std::{iter, process::exit};

async fn handle_outdated_version() -> Result<()> {
    eprintln!(
        "You are using an outdated version of the program. Would you like to update (Y/n/srn/help)?: "
    );
    // Define the versions that are newer than the current versions now, so that it isn't
    // being defined every loop iteration, which is inefficient.
    let newer_releases: Releases = releases.newer_than(current_release)?;
    loop {
        // The input can be y (yes), n (no), srn (see release notes), or help.
        let mut input_type: GetInputMode = GetInputMode::OneOf(vec![
            "y".to_string(),
            "n".to_string(),
            "srn".to_string(),
            "help".to_string(),
        ]);
        // Get the input
        input_type.get_input().await?;
        // Convert the input to a usable type
        let input: String = if let GetInputMode::Normal(input) = input_type {
            input
        } else {
            unreachable!();
        };
        // Match the input
        match input.as_str() {
            // If the input is "y", update to a version the user wants.
            "y" => {
                // Remove the bad versions from releases_from_current because the user
                // shouldn't be able to update to those versions.
                let newer_good_releases: Releases = newer_releases.good_versions(bad_versions);
                // The releases from the current release, but as Strings instead.
                let newer_good_releases_string: Vec<String> = newer_good_releases
                    .iter()
                    .map(|release: &Release| -> String {
                        release.try_as_version().unwrap().to_string()
                    })
                    .collect();
                // Prompt the user
                eprintln!(
                    "To what version do you want to update ({})?: ",
                    newer_good_releases_string.join(", ")
                );
                // The input can be one of the versions released after the current one,
                // excluding bad versions.
                let mut input_type: GetInputMode = GetInputMode::OneOf(newer_good_releases_string);
                // Get the input
                input_type.get_input().await?;
                // Convert the input to an usable type
                let input: Version = if let GetInputMode::Normal(input) = input_type {
                    Version::parse(&input)?
                } else {
                    unreachable!()
                };
                // Update to the version the user entered
                newer_good_releases
                    .find_with_version(&input)
                    .unwrap()
                    .update_to()
                    .await?;
            }
            // If the input is "n", do nothing.
            "n" => {}
            // If the input is "srn", output all the release notes up until the version the
            // user wants.
            "srn" => {
                // The releases from the current release, but as String's instead.
                let newer_releases_string: Vec<String> = newer_releases
                    .iter()
                    .map(|release: &Release| -> String {
                        release.try_as_version().unwrap().to_string()
                    })
                    .collect();
                // Prompt the user
                eprintln!(
                    "Up until what version do you want to see the release notes for ({})?: ",
                    newer_releases_string.join(", ")
                );
                // The input can be one of the versions released in the future.
                let mut input_type: GetInputMode = GetInputMode::OneOf(newer_releases_string);
                // Get the input
                input_type.get_input().await?;
                // Convert the input to an usable type
                let input: Version = if let GetInputMode::Normal(input) = input_type {
                    Version::parse(&input)?
                } else {
                    unreachable!()
                };
                // Iterate over every release in the range the user specified (from the
                // current release to the release entered).
                for release in
                    newer_releases
                        .older_than(input.clone())?
                        .iter()
                        .chain(std::iter::once(
                            newer_releases.find_with_version(&input).unwrap(),
                        ))
                {
                    // If there are release notes
                    if let Some(ref release_notes) = release.body {
                        // Print the notice for the version
                        print!("Release notes for release {}", release.try_as_version()?);
                        // If it's a bad version, attach a note that says the user won't be
                        // able to update to this version.
                        if let Some(reason) = bad_versions.get_reason(&release.try_as_version()?) {
                            print!(
                                "(note: you will not be able to update to this version as it marked as having a critical bug. The attached reason is \"{}\")",
                                reason
                            );
                        };
                        // Print the : and the new line.
                        println!(":");
                        // Print the release notes.
                        termimad::print_text(release_notes);
                    } else {
                        // Alert the user that there are no release notes.
                        print!(
                            "Release {} has no release notes.",
                            release.try_as_version()?
                        );
                        // If the release is a bad version, assure the user that they won't
                        // be able to update to that version.
                        if let Some(reason) = bad_versions.get_reason(&release.try_as_version()?) {
                            print!(
                                " You wouldn't have been able to update to this version anyway as it is listed as having a critical bug because of the reason \"{}\".",
                                reason
                            );
                        };
                        // Print the newline
                        println!();
                    };
                    // Print 10 newlines as a separator between multiple release notes
                    print!("\n\n\n\n\n");
                    print!("\n\n\n\n\n");
                }
                // Continue, to get the user input again, as they still haven't decided
                // whether to update or not.
                continue;
            }
            // If the input is "help", print the usage for all possible commands.
            "h" => {
                eprintln!("y - update to a version of your choosing");
                eprintln!("n - don't update and proceed to the program");
                eprintln!(
                    "srn - see release notes from the current version to the version of your choosing"
                );
                eprintln!("help - bring up this help text");
                // Continue, to get the user input again, as they still haven't decided
                // whether to update or not.
                continue;
            }
            // If the input is not one of the commands, and yet passed the GetInputMode's
            // check.
            _ => bail!(
                "Another valid input was encountered. This was either because the code for GetInputMode was faulty, or beacuse of a new available term added but not accounted for. In either case, this error should never be received."
            ),
        };
        break;
    }
    // Ok.
    Ok(())
}

async fn handle_updates(
    releases: Releases,
    current_release: Release,
    unsafe_versions: UnsafeVersions,
) -> Result<()> {
    // If the current version is an unsafe version, forcefully update.
    if let Some(unsafe_version_reason) =
        unsafe_versions.get_reason(&current_release.try_as_version()?)
    {
        // Warn the user
        epr_wrn(format!(
            "Current version ({}) is listed as unsafe because of the reason \"{}\".",
            current_release.try_as_version()?,
            unsafe_version_reason
        ));
        let Some(first_safe_release) =
            releases.first_safe_after(&current_release.try_as_version()?, &unsafe_versions)
        else {
            epr_wrn("No safe version found.");
            handle_unsafe_version().await?;
            debug!("Continuing to program as per user's request.");
            // Ok.
            return Ok(());
        };
        let first_safe_version: Version = first_safe_release.try_as_version()?;
        epr_dbg("Found a safe version {first_safe_version}! Updating...");
        // Update to the fixed release
        first_safe_release.update_to().await?;
        // Ok.
        return Ok(());
    };

    handle_outdated_version().await?;

    // Ok.
    Ok(())
}

pub async fn updater() -> Result<()> {
    let releases: Releases = Releases::fetch().await?;
    let current_release: &Release = get_current_release(&releases)?;
    let current_version: Version = current_release.try_as_version()?;
    let latest_release: &Release = get_latest_release(&releases)?;
    let latest_version: Version = latest_release.try_as_version()?;
    let unsafe_versions: UnsafeVersions = get_unsafe_versions(latest_release, &releases).await?;

    if current_version > latest_version {
        epr_wrn("The program is on a version newer than the latest released version.");
        // Ok.
        return Ok(());
    };
    if let Some(unsafe_reason) = unsafe_versions.get_reason(&current_release.try_as_version()?) {
        handle_unsafe_version().await?;
    }
    if current_version < latest_version {
        handle_updates().await?;
    }
    // Ok.
    Ok(())
}

async fn get_unsafe_versions(
    latest_release: &Release,
    releases: &Releases,
) -> Result<UnsafeVersions> {
    UnsafeVersions::fetch(latest_release, Some(&releases)).await
}

fn get_latest_release(releases: &Releases) -> Result<&Release> {
    releases.latest().ok_or_eyre(eyre!(
        "The repository {GITHUB_REPO_OWNER}/{GITHUB_REPO_NAME} has no releases."
    ))
}

fn get_current_release(releases: &Releases) -> Result<&Release> {
    let current_version: &str = env!("CARGO_PKG_VERSION");
    releases
        .find_with_version(&current_version.parse()?)
        .ok_or_eyre(eyre!(
            "Current version ({current_version}) is not a release."
        ))
}

/// Shows release notes according to the user's selection.
async fn show_release_notes(
    newer_releases: &Releases,
    bad_versions: &UnsafeVersions,
) -> Result<()> {
    let version_strings: Vec<String> = newer_releases
        .iter()
        .map(|r| r.try_as_version().unwrap().to_string())
        .collect();

    eprintln!(
        "Up until what version do you want to see the release notes for ({})?: ",
        version_strings.join(", ")
    );

    let input = prompt_one_of(&version_strings).await?;
    let target_version = Version::parse(&input)?;

    display_release_notes_up_to(newer_releases, &target_version, bad_versions)?;

    Ok(())
}

/// Displays release notes for all `Release`s up to a specific `Version`.
fn display_release_notes_up_to(
    releases: &Releases,
    target: &Version,
    bad_versions: &UnsafeVersions,
) -> Result<()> {
    assert!(releases.find_with_version(target).is_some());
    let older_releases: Vec<&Release> = releases.older_than(target)?;
    for release in older_releases
        .into_iter()
        .chain(iter::once(releases.find_with_version(target).unwrap()))
    {
        display_release_notes(&release, bad_versions)?;
    }
    // Ok.
    Ok(())
}

async fn handle_unsafe_version(
    version: UnsafeVersion,
    current_release: Release,
    latest_release: Release,
) -> Result<()> {
    let latest_version: Version = latest_release.try_as_version()?;
    let current_version: Version = current_release.try_as_version()?;
    eprintln!(
        "Current version ({}) is marked as unsafe because of the reason \"{}\".",
        current_version,
        version.reason()
    );
    if latest_version == current_version {
        return handle_unsafe_version_with_no_updates().await;
    }
    // todo
    // Ok.
    Ok(())
}

async fn handle_unsafe_version_with_no_updates() -> Result<()> {
    eprint!(
        "You have no safe versions to update to. Downgrading versions is not recommended. It is recommended that you wait for a stable release. Do you still wish to continue (y/N)?: "
    );
    let mut input_type: GetInputMode = GetInputMode::Bool(false);
    input_type.get_input().await?;
    if let GetInputMode::Bool(choice) = input_type
        && !choice
    {
        exit(0);
    }
    // Ok.
    Ok(())
}
