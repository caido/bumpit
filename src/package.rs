use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

use anyhow::Context as _;
use semver::Version;

pub struct Package(toml_edit::DocumentMut);

impl Package {
    pub fn read(path: &Path) -> anyhow::Result<Self> {
        // Read file
        let mut file = File::open(path).context("Failed to open file")?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .context("Failed to read file")?;

        // Parse toml
        let doc = buf.parse::<toml_edit::DocumentMut>()?;

        Ok(Self(doc))
    }

    pub fn set_version(&mut self, version: &Version) {
        self.0["package"]["version"] = toml_edit::value(version.to_string());
    }

    pub fn get_version(&self) -> anyhow::Result<Version> {
        let version_string = self.0["package"]["version"]
            .as_str()
            .context("toml has no version")?;
        version_string
            .parse::<Version>()
            .context("version is not semver")
    }

    pub fn write(&self, path: &Path) -> anyhow::Result<()> {
        let mut file = File::create(path).context("Failed to create file")?;
        file.write_all(self.0.to_string().as_bytes())
            .context("Failed to write file")?;
        Ok(())
    }
}
