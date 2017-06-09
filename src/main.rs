#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate clap;

extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate toml;

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
        let package_data = get_package_data(&client, &package_name);
        println!("{:?}", package_data);
    }
    if let Some(matches) = matches.subcommand_matches("pipfile-info") {
        let pipfile_bytes = get_file_path_bytes(matches.value_of("PIPFILE_PATH").unwrap()).unwrap();

        let pipfile_inst: pipfile::Pipfile = toml::from_slice(&pipfile_bytes)
            .expect("failed to parse Pipfile");

        for (package_name, _) in pipfile_inst.packages {
            println!("{}", package_name);
            let _ = get_package_data(&client, &package_name).unwrap();
        }
    }
}

