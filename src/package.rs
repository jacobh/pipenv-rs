use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Package {
    info: PackageInfo,
    releases: HashMap<String, Vec<ReleaseMetadata>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct PackageInfo {
    maintainer: Option<String>,
    docs_url: Option<String>,
    requires_python: Option<String>,
    maintainer_email: Option<String>,
    keywords: Option<String>,
    package_url: String,
    author: String,
    author_email: String,
    download_url: String,
    platform: String,
    version: String,
    description: String,
    release_url: String,
    downloads: PackageDownloads,
    requires_dist: Option<Vec<String>>,
    classifiers: Vec<String>,
    name: String,
    bugtrack_url: Option<String>,
    license: String,
    summary: String,
    home_page: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct PackageDownloads {
    last_month: u64,
    last_week: u64,
    last_day: u64,
}

#[derive(Serialize, Deserialize, Debug)]
struct ReleaseMetadata {
    has_sig: bool,
    upload_time: String,
    comment_text: String,
    python_version: String,
    url: String,
    md5_digest: String,
    downloads: u64,
    filename: String,
    packagetype: String,
    path: String,
    size: u64,
}

