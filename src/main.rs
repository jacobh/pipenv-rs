#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate clap;

extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate toml;
extern crate semver;

use std::fs::File;
use std::io::Read;

mod pipfile;
mod pypi;

fn get_package_data(client: &reqwest::Client,
                    package_name: &str)
                    -> reqwest::Result<pypi::PypiPackage> {

    let mut resp = client
        .get(&format!("https://pypi.python.org/pypi/{}/json", package_name))
        .send()?;
    Ok(resp.json()?)
}

fn get_file_path_bytes(path: &str) -> std::io::Result<Vec<u8>> {
    let mut bytes = vec![];
    let mut file = File::open(path)?;
    file.read_to_end(&mut bytes)?;
    Ok(bytes)
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = clap::App::from_yaml(yaml).get_matches();
    let client = reqwest::Client::new().unwrap();

    if let Some(matches) = matches.subcommand_matches("info") {
        let package_name = matches.value_of("PACKAGE_NAME").unwrap();
        let package_data = get_package_data(&client, &package_name).unwrap();
        println!("latest version: {:?}", package_data.latest_version());
    }
    if let Some(matches) = matches.subcommand_matches("pipfile-info") {
        let pipfile_bytes = get_file_path_bytes(matches.value_of("PIPFILE_PATH").unwrap()).unwrap();

        let pipfile_inst: pipfile::Pipfile = toml::from_slice(&pipfile_bytes)
            .expect("failed to parse Pipfile");

        let package_data =
            pipfile_inst
                .packages
                .keys()
                .map(|package_name| get_package_data(&client, package_name).unwrap());
        for package_datum in package_data {
            println!("{}", package_datum.name());
            println!("latest version: {:?}", package_datum.latest_version());
        }
    }
    if let Some(matches) = matches.subcommand_matches("validate-lockfile") {
        let lockfile_bytes = get_file_path_bytes(matches.value_of("LOCKFILE_PATH").unwrap())
            .unwrap();
        let lockfile: pipfile::Lockfile = serde_json::from_slice(&lockfile_bytes)
            .expect("failed to parse Pipfile.lock");
        println!("ok");
    }
}

