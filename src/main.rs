#[macro_use]
extern crate serde_derive;

extern crate reqwest;
extern crate serde;
extern crate serde_json;

use std::env;

mod package;

fn get_package_data(client: &reqwest::Client,
                    package_name: &str)
                    -> reqwest::Result<package::Package> {

    let mut resp = client
        .get(&format!("https://pypi.python.org/pypi/{}/json", package_name))
        .send()?;
    Ok(resp.json()?)
}

fn main() {
    let package_name = env::args()
        .nth(1)
        .expect("argument <PACKAGE_NAME> required");

    let client = reqwest::Client::new().unwrap();

    let package_data = get_package_data(&client, &package_name);
    println!("{:?}", package_data);

}
