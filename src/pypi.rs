use std::collections::HashMap;
use std::io::Read;
use std::str;
use semver;
use reqwest;
use flate2::read::GzDecoder;
use tar::Archive;

use release_utils::get_tar_archive;
use semver_utils::normalize_version_string;

#[derive(Deserialize, Debug)]
pub struct PypiPackage {
    info: PackageInfo,
    releases: HashMap<String, Vec<ReleaseMetadata>>,
    urls: Vec<ReleaseMetadata>,
}
impl PypiPackage {
    pub fn get_requires_for_version(&self,
                                    client: &reqwest::Client,
                                    version: &semver::Version)
                                    -> Option<Vec<String>> {
        let sdist_release = self.releases()
            .get(version)
            .unwrap()
            .iter()
            .find(|release| release.package_type == ReleaseType::Sdist)
            .unwrap();
        sdist_release.get_requires(client)
    }

    pub fn releases(&self) -> HashMap<semver::Version, &Vec<ReleaseMetadata>> {
        self.releases
            .iter()
            .filter_map(|(key, value)| {
                            semver::Version::parse(&normalize_version_string(key))
                                .map(|key| (key, value))
                                .ok()
                        })
            .collect()
    }

    pub fn latest_version(&self) -> Option<semver::Version> {
        self.releases().keys().max().map(|x| x.to_owned())
    }

    pub fn name(&self) -> &str {
        &self.info.name
    }
}

#[derive(Deserialize, Debug)]
struct PackageInfo {
    maintainer: Option<String>,
    docs_url: Option<String>,
    requires_python: Option<String>,
    maintainer_email: Option<String>,
    keywords: Option<String>,
    package_url: String,
    author: String,
    author_email: String,
    download_url: Option<String>,
    platform: String,
    version: String,
    description: String,
    release_url: String,
    downloads: PackageDownloads,
    requires_dist: Option<Vec<String>>,
    classifiers: Vec<String>,
    name: String,
    bugtrack_url: Option<String>,
    license: Option<String>,
    summary: String,
    home_page: String,
}

#[derive(Deserialize, Debug)]
struct PackageDownloads {
    last_month: u64,
    last_week: u64,
    last_day: u64,
}

#[derive(Deserialize, Debug)]
pub struct ReleaseMetadata {
    has_sig: bool,
    upload_time: String,
    comment_text: String,
    python_version: String,
    url: String,
    md5_digest: String,
    downloads: u64,
    filename: String,
    #[serde(rename = "packagetype")]
    package_type: ReleaseType,
    path: String,
    size: u64,
}
impl ReleaseMetadata {
    fn get_release_file(&self, client: &reqwest::Client) -> reqwest::Result<reqwest::Response> {
        client.get(&self.url).send()
    }
    fn get_requires(&self, client: &reqwest::Client) -> Option<Vec<String>> {
        let resp = self.get_release_file(client).unwrap();
        match self.package_type {
            ReleaseType::Sdist => {
                let mut archive = Archive::new(GzDecoder::new(resp).unwrap());
                for entry in archive.entries().unwrap() {
                    let mut entry = entry.unwrap();
                    if entry
                           .path()
                           .unwrap()
                           .to_string_lossy()
                           .ends_with(".egg-info/requires.txt") {
                        let requires_txt = {
                            let mut data = String::new();
                            entry
                                .read_to_string(&mut data)
                                .expect("failed to read requires.txt");
                            data
                        };
                        return Some(requires_txt.split("\n").map(|x| x.to_owned()).collect());
                    }
                }
                None
            }
            _ => unimplemented!(),
        }
    }
}

#[derive(Deserialize, Debug, PartialEq)]
enum ReleaseType {
    #[serde(rename = "sdist")]
    Sdist,
    #[serde(rename = "bdist_dumb")]
    BdistDumb,
    #[serde(rename = "bdist_egg")]
    BdistEgg,
    #[serde(rename = "bdist_wheel")]
    BdistWheel,
    #[serde(rename = "bdist_wininst")]
    BdistWininst,
}

