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

mod pypi;

fn get_package_data(client: &reqwest::Client,
                    package_name: &str)
                    -> reqwest::Result<pypi::PypiPackage> {

    let mut resp = client
        .get(&format!("https://pypi.python.org/pypi/{}/json", package_name))
        .send()?;
    Ok(resp.json()?)
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
        let mut pipfile_bytes = vec![];
        {
            let mut pipfile_file = File::open(matches.value_of("PIPFILE_PATH").unwrap())
                .expect("Pipfile path does not point to file");
            pipfile_file
                .read_to_end(&mut pipfile_bytes)
                .expect("failed to read Pipfile");
        }

        let pipfile_data: toml::Value = toml::from_slice(&pipfile_bytes)
            .expect("failed to parse Pipfile");

        for (package_name, _) in pipfile_data["packages"].as_table().unwrap() {
            println!("{}", package_name);
            let _ = get_package_data(&client, &package_name).unwrap();
        }
    }

}

