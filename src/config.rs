use std::path::PathBuf;

use anyhow::Context;
use cargo_metadata::MetadataCommand;
use clap::Parser;

use crate::modifier::{BumpKind, VersionModifier};

#[derive(Parser, Debug)]
#[clap(name = "Caido", about)]
pub struct Arguments {
    /// Version should be a semver (https://semver.org/) string or the position of the current version to increment: premajor, major, preminor, minor, prepatch, patch, prerelease.
    #[arg(index = 1)]
    pub version: BumpKind,

    /// Optional path to Cargo.toml
    #[arg(long, value_name = "PATH")]
    pub manifest_path: Option<String>,

    /// Optional pre-release information.
    #[arg(short = 'p', long, value_name = "PRE-RELEASE ID")]
    pub pre_id: Option<String>,

    /// Optional package name
    #[arg(short = 'k', long, value_name = "PACKAGE NAME")]
    pub package: Option<String>,
}

pub struct Config {
    pub modifier: VersionModifier,
    pub manifest_path: PathBuf,
}

impl Config {
    pub fn new(args: Arguments) -> anyhow::Result<Self> {
        let mut metadata_cmd = MetadataCommand::new();
        if let Some(path) = args.manifest_path {
            metadata_cmd.manifest_path(path);
        }

        let target_package = args.package.unwrap_or(".".to_string());
        let metadata = metadata_cmd
            .exec()
            .context("Failed to get cargo metadata")?;
        let package = metadata
            .packages
            .iter()
            .find(|package| package.name.as_ref() == target_package)
            .context("Package does not exist")?;
        metadata
            .workspace_members
            .iter()
            .find(|member_id| package.id.eq(member_id))
            .context("Package is not a workspace member")?;

        Ok(Self {
            modifier: VersionModifier::new(args.version, args.pre_id),
            manifest_path: package.manifest_path.clone().into_std_path_buf(),
        })
    }
}
