//! SPDX-License-Identifier: GPL-3.0-only

use crate::{enums::ReleaseSafety, structs::Releases, utils::epr_wrn};
use color_eyre::eyre::{Result, bail};
use tokio::io::{self, AsyncWriteExt};

#[derive(Debug)]
pub struct Updater {
    releases: Releases,
}

impl Updater {
    pub async fn try_new() -> Result<Self> {
        let releases: Releases = Releases::new().fetch().await?;

        // Ok.
        Ok(Self { releases })
    }
}

impl Updater {
    pub async fn update(&self) -> Result<()> {
        if *self.releases.current().await?.version() > *self.releases.latest()?.version() {
            epr_wrn("The program is on a version newer than the latest released version.");
        }


        if self.releases.current().await?.is_safe() {
            self.safe_version().await?
        }else {
            self.unsafe_version().await?
            if *self.releases.current().await?.version() < *self.releases.latest()?.version() {
                todo!()
            } else {
                epr_wrn("There are no available releases to update to.");
                match self.releases.current().await?.safety() {
                    ReleaseSafety::Safe => unreachable!(),
                    ReleaseSafety::Unsafe { .. } => eprintln!("Playing on an unsafe version is not recommended")
                }
            }
        }
    }
    async fn safe_version(&self) -> Result<()> {
        assert!(self.releases.current().await?.is_safe());

        if *self.releases.current().await?.version() < *self.releases.latest()?.version() {
            todo!()
        }

        // Ok.
        Ok(())
    }
    async fn unsafe_version(&self) -> Result<()> {
        assert!(!self.releases.current().await?.is_safe());

        epr_wrn(format!("You are on a version marked as unsafe (severity: {}) because of the reason \"{}\".", self.releases.current().await?.safety(), self.releases.current().await?.safety().reason().unwrap()));

        if *self.releases.current().await?.version() < *self.releases.latest()?.version() {
            todo!()
        }
        else {
            epr_wrn("No newer release available.");
            match self.releases.current().await?.safety() {
                ReleaseSafety::Safe => unreachable!(),
                ReleaseSafety::Unsafe { .. } => todo!(),
                ReleaseSafety::UpdateUnsafe { .. } => {
                    eprintln!("The code responsible for updating is marked as unsafe. Everything else should work normally.");
                    eprintln!("Keep in mind that you will have to look out for updates yourself.");
                    eprintln!("Press any key to continue...");
                    io::stderr().flush().unwrap();

                },
                ReleaseSafety::ReallyUnsafe { .. } => bail!("The severity level is ReallyUnsafe."),
            }
        }

        // Ok.
        Ok(())
    }
}
