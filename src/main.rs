#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate clap;

extern crate reqwest;
extern crate serde;
extern crate serde_json;

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
    let yaml = load_yaml!("cli.yml");
    let matches = clap::App::from_yaml(yaml).get_matches();
    let client = reqwest::Client::new().unwrap();

    if let Some(matches) = matches.subcommand_matches("info") {
        let package_name = matches.value_of("PACKAGE_NAME").unwrap();
        let package_data = get_package_data(&client, &package_name);
        println!("{:?}", package_data);
    }


}
