use std::collections::HashMap;

type RequiresMap = HashMap<String, String>;

#[derive(Deserialize, Debug)]
pub struct Pipfile {
    pub source: Vec<Source>,
    pub requires: RequiresMap,
    pub packages: HashMap<String, PackageInfo>,
    #[serde(rename = "dev-packages")]
    pub dev_packages: Option<HashMap<String, PackageInfo>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Source {
    url: String,
    verify_ssl: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum PackageInfo {
    SimpleString(String),
    Simple {
        version: String,
        hash: Option<String>,
    },
    Git {
        git: String,
        #[serde(rename = "ref")]
        ref_: String,
        #[serde(default = "git_editable_default")]
        editable: bool,
    },
}

fn git_editable_default() -> bool {
    false
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Lockfile {
    default: HashMap<String, PackageInfo>,
    develop: HashMap<String, PackageInfo>,
    _meta: LockfileMeta,
}

#[derive(Serialize, Deserialize, Debug)]
struct LockfileMeta {
    hash: LockfileMetaHash,
    requires: RequiresMap,
    sources: Vec<Source>,
}

#[derive(Serialize, Deserialize, Debug)]
struct LockfileMetaHash {
    sha256: String,
}

