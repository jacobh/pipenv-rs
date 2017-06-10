use std::collections::HashMap;
use serde_json;

#[derive(Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum ReleaseType {
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

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct WheelMetadata {
    #[serde(default)]
    keywords: Vec<String>,
    classifiers: Vec<String>,
    extensions: HashMap<String, serde_json::Value>,
    #[serde(default)]
    extras: Vec<String>,
    generator: String,
    license: Option<String>,
    metadata_version: String,
    name: String,
    requires: Option<String>,
    #[serde(default)]
    run_requires: Vec<WheelRequiresGroup>,
    #[serde(default)]
    test_requires: Vec<WheelRequiresGroup>,
    summary: String,
    version: String,
    download_url: Option<String>,
    platform: Option<String>,
    provides: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct WheelRequiresGroup {
    extra: Option<String>,
    environment: Option<String>,
    requires: Vec<String>,
}

