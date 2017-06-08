#[macro_use]
extern crate serde_derive;

extern crate reqwest;
extern crate serde;
extern crate serde_json;

use std::env;

mod package;

fn main() {
    let package_name = env::args()
        .nth(1)
        .expect("argument <PACKAGE_NAME> required");

    let client = reqwest::Client::new().unwrap();

    let mut resp = client
        .get(&format!("https://pypi.python.org/pypi/{}/json", package_name))
        .send()
        .unwrap();
    let package_data: package::Package = resp.json().unwrap();
    println!("{:?}", package_data);
}
