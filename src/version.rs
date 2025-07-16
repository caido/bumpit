use semver::Version;

pub trait VersionExt {
    fn increment_major(&mut self);

    fn increment_minor(&mut self);

    fn increment_patch(&mut self);
}

impl VersionExt for Version {
    fn increment_major(&mut self) {
        self.major += 1;
        self.minor = 0;
        self.patch = 0;
        self.pre = semver::Prerelease::EMPTY;
        self.build = semver::BuildMetadata::EMPTY;
    }

    fn increment_minor(&mut self) {
        self.minor += 1;
        self.patch = 0;
        self.pre = semver::Prerelease::EMPTY;
        self.build = semver::BuildMetadata::EMPTY;
    }

    fn increment_patch(&mut self) {
        self.patch += 1;
        self.pre = semver::Prerelease::EMPTY;
        self.build = semver::BuildMetadata::EMPTY;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increment_major() {
        let mut version = Version::parse("0.1.0").unwrap();
        version.increment_major();
        assert_eq!(version, Version::parse("1.0.0").unwrap());
    }

    #[test]
    fn test_increment_minor() {
        let mut version = Version::parse("0.1.0").unwrap();
        version.increment_minor();
        assert_eq!(version, Version::parse("0.2.0").unwrap());
    }

    #[test]
    fn test_increment_patch() {
        let mut version = Version::parse("0.1.0").unwrap();
        version.increment_patch();
        assert_eq!(version, Version::parse("0.1.1").unwrap());
    }
}
