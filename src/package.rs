use serde_json;
use std::collections::HashMap;

type SerdeObject = serde_json::Map<String, serde_json::Value>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Package {
    info: SerdeObject,
    releases: HashMap<String, Vec<ReleaseMetadata>>,
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
