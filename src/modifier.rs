use std::str::FromStr;

use anyhow::Context as _;
use semver::Version;

use crate::version::VersionExt;

#[derive(Debug, PartialEq, Clone)]
pub enum BumpKind {
    Replace(Version),
    PreMajor,
    Major,
    PreMinor,
    Minor,
    PrePatch,
    Patch,
    PreRelease,
}

impl FromStr for BumpKind {
    type Err = anyhow::Error;
    fn from_str(input: &str) -> Result<BumpKind, Self::Err> {
        Ok(match input {
            "premajor" => BumpKind::PreMajor,
            "major" => BumpKind::Major,
            "preminor" => BumpKind::PreMinor,
            "minor" => BumpKind::Minor,
            "prepatch" => BumpKind::PrePatch,
            "patch" => BumpKind::Patch,
            "prerelease" => BumpKind::PreRelease,
            _ => BumpKind::Replace(Version::parse(input).context("Invalid version")?),
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct VersionModifier {
    kind: BumpKind,
    pre_id: Option<String>,
}

impl VersionModifier {
    pub fn new(kind: BumpKind, pre_id: Option<String>) -> Self {
        Self { kind, pre_id }
    }

    pub fn apply(&self, version: &mut Version) -> anyhow::Result<()> {
        match &self.kind {
            BumpKind::Replace(replacement) => {
                *version = replacement.clone();
            }
            BumpKind::PreMajor => {
                version.increment_major();
                version.pre = build_prerelease(self.pre_id.as_deref(), 0)?;
            }
            BumpKind::Major => {
                if version.pre.is_empty() {
                    version.increment_major();
                } else {
                    version.pre = semver::Prerelease::EMPTY;
                }
            }
            BumpKind::PreMinor => {
                version.increment_minor();
                version.pre = build_prerelease(self.pre_id.as_deref(), 0)?;
            }
            BumpKind::Minor => {
                if version.pre.is_empty() {
                    version.increment_minor();
                } else {
                    version.pre = semver::Prerelease::EMPTY;
                }
            }
            BumpKind::PrePatch => {
                version.increment_patch();
                version.pre = build_prerelease(self.pre_id.as_deref(), 0)?;
            }
            BumpKind::Patch => {
                if version.pre.is_empty() {
                    version.increment_patch();
                } else {
                    version.pre = semver::Prerelease::EMPTY;
                }
            }
            BumpKind::PreRelease => {
                if version.pre.is_empty() {
                    version.increment_patch();
                }
                let (pre_id, numeric) = parse_prerelease(&version.pre)?;
                version.pre = build_prerelease(Some(&pre_id), numeric.map(|n| n + 1).unwrap_or(0))?;
            }
        }
        Ok(())
    }
}

fn build_prerelease(pre_id: Option<&str>, numeric: u64) -> anyhow::Result<semver::Prerelease> {
    let raw = if let Some(pre_id) = pre_id {
        format!("{}.{}", pre_id, numeric)
    } else {
        numeric.to_string()
    };
    semver::Prerelease::new(&raw).context("Invalid prerelease identifier")
}

fn parse_prerelease(pre: &semver::Prerelease) -> anyhow::Result<(String, Option<u64>)> {
    if let Some((pre_id, numeric)) = pre.as_str().split_once('.') {
        let pre_id = pre_id.to_owned();
        let numeric = u64::from_str(numeric)
                .map_err(|_| anyhow::anyhow!("This version scheme is not supported. Use format like `pre`, `dev` or `alpha.1` for prerelease symbol"))?;
        Ok((pre_id, Some(numeric)))
    } else {
        Ok((pre.as_str().to_owned(), None))
    }
}
