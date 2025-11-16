/// SPDX-License-Identifier: GPL-3.0-only
use crate::{
    consts::{GITHUB_REPO_NAME, GITHUB_REPO_OWNER},
    enums::GetInputMode,
    structs::{BadVersions, Release, Releases},
};
use color_eyre::eyre::{self, Context, OptionExt, Result, bail, eyre};
use semver::Version;

pub async fn updater() -> Result<()> {
    // Get all releases
    let releases: Releases = {
        let mut releases: Releases = Releases::fetch().await?;
        releases.sort_unstable_by_version()?;
        releases
    };
    // If the current version isn't in releases, return an error
    let Some(current_release) = releases.find_with_version(&env!("CARGO_PKG_VERSION").parse()?)
    else {
        bail!(
            "Current version ({}) is not a release.",
            env!("CARGO_PKG_VERSION")
        );
    };
    // Get the latest release
    let latest_release: &Release = releases.0.first().ok_or_else(|| {
        eyre!("The repository {GITHUB_REPO_OWNER}/{GITHUB_REPO_NAME} has no releases.")
    })?;
    // Get bad versions
    let bad_versions: BadVersions = BadVersions::fetch(Some(&releases))?;
    // If the current version matches the latest...
    if current_release.try_as_version()? == latest_release.try_as_version()? {
        // If the current version is a bad version, ask the user how to proceed
        if let Some(bad_version_reason) =
            bad_versions.get_reason(&current_release.try_as_version()?)
        {
            // Prompt the user
            eprintln!(
                "Your program is on the latest version, but that version is listed as unsafe to use because of reason \"{}\". It is recommended to wait for a stable release. Do you still wish to continue (y/N)?: ",
                bad_version_reason
            );
            // The input will be a bool
            let mut input_type: GetInputMode = GetInputMode::Bool(false);
            // Get the input
            input_type.get_input().await?;
            // Convert the input to a usable type
            let input: bool = if let GetInputMode::Bool(choice) = input_type {
                choice
            } else {
                unreachable!();
            };
            // If the user said they do not wish to proceed, exit with a reassuring message.
            if !input {
                bail!(
                    "Successful exit. Ignore the fact it says \"Error\" at the beginning of the line. No, really, don't worry about it."
                );
            };
        };
    } else if current_release.try_as_version()? > latest_release.try_as_version()? {
        warn!("The program is on a version newer than the latest released version.");
    } else {
        // If the current version is a bad version, forcefully update.
        if let Some(bad_version_reason) =
            bad_versions.get_reason(&current_release.try_as_version()?)
        {
            debug!(
                "Current ({}) version is listed as having a critical bug because of reason \"{}\".",
                current_release.try_as_version()?,
                bad_version_reason
            );
            // Get the version that will fix the bug.
            let mut fixed_release: &Release = current_release;
            loop {
                // Get the position of the current release, and get the previous element of that,
                // in other words, the release newer than the current one, and set that to the
                // value of fixed_release.
                fixed_release = releases.0.get(
                    releases
                        .0
                        .iter()
                        .position(|release: &Release| -> bool {
                            release.try_as_version().unwrap() == fixed_release.try_as_version().unwrap()
                        })
                        .expect("This will only fail if the current version is not a release, which should've already been checked using BadVersions::check().")
                        - 1,
                ).expect("This will only fail if the current_version is the first element in releases, but this has already been checked.");
                // If the fixed_release is actually a bad version, iterate again (that will work
                // because then the next iteration will find the release newer than the supposed
                // fixed release, and this will repeat until a release that isn't a bad one is
                // found).
                if bad_versions
                    .get_reason(&fixed_release.try_as_version()?)
                    .is_some()
                {
                    continue;
                } else {
                    break;
                };
            }
            // Warn the user
            {
                let warning_msg: String = format!(
                    "Current version {} is marked as having a critical bug because of reason \"{}\". Updating to the first version that fixes it ({})...",
                    current_release.try_as_version()?,
                    bad_version_reason,
                    fixed_release.try_as_version()?
                );
                warn!("{}", warning_msg);
                eprintln!("{}", warning_msg);
            };
            // Update to the fixed release
            fixed_release.update_to().await?;
        } else {
            // If the current version is not a bad version, just outdated.
            eprintln!(
                "You are using an outdated version of the program. Would you like to update (Y/n/srn/help)?: "
            );
            // Define the versions that are newer than the current versions now, so that it isn't
            // being defined every loop iteration, which is inefficient.
            let releases_from_current: Releases = releases.newer_than(current_release)?;
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
                        let releases_from_current: Releases = Releases(
                            releases_from_current
                                .0
                                .into_iter()
                                .filter(|release: &Release| -> bool {
                                    bad_versions
                                        .get_reason(&release.try_as_version().unwrap())
                                        .is_none()
                                })
                                .collect(),
                        );
                        // The releases from the current release, but as String's instead.
                        let releases_from_current_str: Vec<String> = releases_from_current
                            .0
                            .iter()
                            .map(|release: &Release| -> String {
                                release.try_as_version().unwrap().to_string()
                            })
                            .collect();
                        // Prompt the user
                        eprintln!(
                            "To what version do you want to update ({})?: ",
                            releases_from_current_str.join(", ")
                        );
                        // The input can be one of the versions released after the current one,
                        // excluding bad versions.
                        let mut input_type: GetInputMode =
                            GetInputMode::OneOf(releases_from_current_str);
                        // Get the input
                        input_type.get_input().await?;
                        // Convert the input to an usable type
                        let input: Version = if let GetInputMode::Normal(input) = input_type {
                            Version::parse(&input)?
                        } else {
                            unreachable!()
                        };
                        // Update to the version the user entered
                        releases_from_current
                            .find_with_version(&input)
                            .unwrap()
                            .update_to()
                            .await?;
                    }
                    // If the input is "n", do nothing.
                    "n" => {}
                    // If the input is "srn", output all the release notes up until the version the
                    // uesr wants.
                    "srn" => {
                        // The releases from the current release, but as String's instead.
                        let releases_from_current_str: Vec<String> = releases_from_current
                            .0
                            .iter()
                            .map(|release: &Release| -> String {
                                release.try_as_version().unwrap().to_string()
                            })
                            .collect();
                        // Prompt the user
                        eprintln!(
                            "Up until what version do you want to see the release notes for ({})?: ",
                            releases_from_current_str.join(", ")
                        );
                        // The input can be one of the versions released in the future.
                        let mut input_type: GetInputMode =
                            GetInputMode::OneOf(releases_from_current_str);
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
                        for release in releases_from_current
                            .older_than(input.clone())?
                            .0
                            .iter()
                            .chain(std::iter::once(
                                releases_from_current.find_with_version(&input).unwrap(),
                            ))
                        {
                            // If there are release notes
                            if let Some(ref release_notes) = release.body {
                                // Print the notice for the version
                                print!("Release notes for release {}", release.try_as_version()?);
                                // If it's a bad version, attach a note that says the user won't be
                                // able to update to this version.
                                if let Some(reason) =
                                    bad_versions.get_reason(&release.try_as_version()?)
                                {
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
                                if let Some(reason) =
                                    bad_versions.get_reason(&release.try_as_version()?)
                                {
                                    print!(
                                        " You wouldn't have been able to update to this version anyway as it is listed as having a critical bug because of the reason \"{}\".",
                                        reason
                                    );
                                };
                                // Print the newline
                                print!("\n");
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
            todo!()
        };
    };
    // Ok.
    Ok(())
}
