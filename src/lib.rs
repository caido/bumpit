mod config;
mod modifier;
mod package;
mod version;

pub use config::Arguments;

pub fn apply(arguments: config::Arguments) -> anyhow::Result<()> {
    let config = config::Config::new(arguments)?;

    // Read package
    let mut package = package::Package::read(&config.manifest_path)?;
    let mut version = package.get_version()?;

    // Bump version
    config.modifier.apply(&mut version)?;
    package.set_version(&version);

    // Write package
    package.write(&config.manifest_path)?;

    Ok(())
}
