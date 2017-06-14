use std::collections::HashMap;
use std::str;
use semver;
use reqwest;

use parse_release::parse_release_requirements;
use semver_utils::normalize_and_parse_version_string;
use version_req::PackageVersionReq;

use release::ReleaseType;

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
                                    -> Option<Vec<PackageVersionReq>> {
        let bdist_release = self.releases()
            .get(version)
            .unwrap()
            .iter()
            .find(|release| release.package_type == ReleaseType::BdistWheel);
        match bdist_release {
            Some(bdist_release) => bdist_release.get_requires(client),
            None => None,
        }

    }

    pub fn releases(&self) -> HashMap<semver::Version, &Vec<ReleaseMetadata>> {
        self.releases
            .iter()
            .map(|(key, value)| (normalize_and_parse_version_string(key), value))
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
    #[serde(default)]
    requires_dist: Vec<String>,
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
    fn get_requires(&self, client: &reqwest::Client) -> Option<Vec<PackageVersionReq>> {
        let resp = self.get_release_file(client).unwrap();
        Some(parse_release_requirements(resp, self.package_type).unwrap())
    }
}

