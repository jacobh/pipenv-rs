use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct PypiPackage {
    info: PackageInfo,
    releases: HashMap<String, Vec<ReleaseMetadata>>,
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
struct ReleaseMetadata {
    has_sig: bool,
    upload_time: String,
    comment_text: String,
    python_version: String,
    url: String,
    md5_digest: String,
    downloads: u64,
    filename: String,
    packagetype: ReleaseType,
    path: String,
    size: u64,
}

#[derive(Deserialize, Debug)]
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

