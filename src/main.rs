use clap::Parser;

mod config;
mod modifier;
mod package;
mod version;

fn main() {
    // Parse arguments
    let cli = config::Arguments::parse();
    let config = config::Config::new(cli).unwrap();

    // Read package
    let mut package = package::Package::read(&config.manifest_path).unwrap();
    let mut version = package.get_version().unwrap();

    // Bump version
    config.modifier.apply(&mut version).unwrap();
    package.set_version(&version);

    // Write package
    package.write(&config.manifest_path).unwrap();
}
