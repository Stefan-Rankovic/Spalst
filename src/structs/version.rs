use color_eyre::eyre::{Result, WrapErr, bail, eyre};
use std::{cmp::Ordering, fmt, str::FromStr};

pub struct Version {
    major: u8,
    minor: u8,
    patch: u8,
}

// Manual implementations for PartialEq, Eq, PartialOrd, and Ord, because in the future the Version
// struct may support version names like "v1.0.0-alpha".

impl PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        self.major == other.major && self.minor == other.minor && self.patch == other.patch
    }
}
impl Eq for Version {}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let major_cmp: Option<Ordering> = self.major.partial_cmp(&other.major);
        if major_cmp != Some(Ordering::Equal) {
            return major_cmp;
        };
        let minor_cmp: Option<Ordering> = self.minor.partial_cmp(&other.minor);
        if minor_cmp != Some(Ordering::Equal) {
            return minor_cmp;
        };
        self.patch.partial_cmp(&other.patch)
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        let major_cmp: Ordering = self.major.cmp(&other.major);
        if major_cmp != Ordering::Equal {
            return major_cmp;
        };
        let minor_cmp: Ordering = self.minor.cmp(&other.minor);
        if minor_cmp != Ordering::Equal {
            return minor_cmp;
        };
        self.patch.cmp(&other.patch)
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "v{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl FromStr for Version {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self> {
        let no_v: &str = s.strip_prefix('v').unwrap_or(s);
        let mut parts: std::str::Split<'_, char> = no_v.split('.');
        let major: u8 = parts
            .next()
            .ok_or_else(|| eyre!("Missing major version in provided string ({s})."))?
            .parse()
            .wrap_err_with(|| format!("Invalid major version in provided string ({s})."))?;
        let minor: u8 = parts
            .next()
            .ok_or_else(|| eyre!("Missing minor version in provided string ({s})."))?
            .parse()
            .wrap_err_with(|| format!("Invalid minor version in provided string ({s})."))?;
        let patch: u8 = parts
            .next()
            .ok_or_else(|| eyre!("Missing patch version in provided string ({s})."))?
            .parse()
            .wrap_err_with(|| format!("Invalid patch version in provided string ({s})."))?;
        if parts.next().is_some() {
            bail!("Too many version components in provided string ({s}).");
        };
        Ok(Self {
            major,
            minor,
            patch,
        })
    }
}
