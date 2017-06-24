#![recursion_limit = "1024"]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate error_chain;

extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate toml;
extern crate semver;
extern crate regex;
extern crate tar;
extern crate flate2;
extern crate rayon;
extern crate zip;

use std::fs::File;
use std::io::{Read, Write, stdout};
use rayon::prelude::*;

mod pipfile;
mod pypi;
mod parse_release;
mod release;
mod semver_utils;
mod version_req;
mod errors;

use errors::*;

fn main() {
    if let Err(ref e) = run() {
        use std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "error: {}", e).expect(errmsg);

        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(errmsg);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
        }

        ::std::process::exit(1);
    }
}

fn get_package_data(client: &reqwest::Client, package_name: &str) -> Result<pypi::PypiPackage> {

    let mut resp = client
        .get(&format!(
            "https://pypi.python.org/pypi/{}/json",
            package_name
        ))
        .send()?;
    Ok(resp.json()?)
}

fn get_file_path_bytes(path: &str) -> Result<Vec<u8>> {
    let mut bytes = vec![];
    let mut file = File::open(path)?;
    file.read_to_end(&mut bytes)?;
    Ok(bytes)
}

fn run() -> Result<()> {
    let yaml = load_yaml!("cli.yml");
    let matches = clap::App::from_yaml(yaml).get_matches();
    let client = reqwest::Client::new().unwrap();

    if let Some(matches) = matches.subcommand_matches("info") {
        let package_name = matches.value_of("PACKAGE_NAME").unwrap();
        let package_data = get_package_data(&client, &package_name)?;
        let latest_version = package_data.latest_version()?;
        println!("latest version: {:?}", latest_version);
        println!(
            "{:?}",
            package_data
                .get_requires_for_version(&client, &latest_version)?
        );
    }
    if let Some(matches) = matches.subcommand_matches("pipfile-info") {
        let pipfile_bytes = get_file_path_bytes(matches.value_of("PIPFILE_PATH").unwrap())?;

        let pipfile_inst: pipfile::Pipfile = toml::from_slice(&pipfile_bytes)
            .chain_err(|| "failed to parse Pipfile")?;

        pipfile_inst
            .packages
            .par_iter()
            .map(|(k, _)| k)
            .map(|package_name| {
                get_package_data(&client, package_name).unwrap()
            })
            .map(|package_datum| {
                let latest_version = package_datum.latest_version()?;
                let requires = package_datum
                    .get_requires_for_version(&client, &latest_version)?;

                let stdout_ = stdout();
                let mut handle = stdout_.lock();
                writeln!(handle, "{}", package_datum.name()).unwrap();
                writeln!(handle, "latest version: {:?}", latest_version).unwrap();
                writeln!(handle, "{:?}", requires).unwrap();

                Ok(())
            })
            .reduce_with(|r1: Result<()>, r2: Result<()>| r1.and(r2).and(Ok(())))
            .unwrap_or(Ok(()))?;
    }
    if let Some(matches) = matches.subcommand_matches("validate-lockfile") {
        let lockfile_bytes = get_file_path_bytes(matches.value_of("LOCKFILE_PATH").unwrap())?;
        let _: pipfile::Lockfile = serde_json::from_slice(&lockfile_bytes)
            .chain_err(|| "failed to parse Pipfile.lock")?;
        println!("ok");
    }
    Ok(())
}
