use std::collections::HashMap;


#[derive(Deserialize, Debug)]
pub struct Pipfile {
    pub source: Vec<Source>,
    pub requires: HashMap<String, String>,
    pub packages: HashMap<String, PackageInfo>,
    #[serde(rename = "dev-packages")]
    pub dev_packages: Option<HashMap<String, PackageInfo>>,
}

#[derive(Deserialize, Debug)]
pub struct Source {
    url: String,
    verify_ssl: bool,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum PackageInfo {
    SemVer(String),
    Git {
        git: String,
        #[serde(rename = "ref")]
        ref_: String,
        #[serde(default = "package_info_git_editable_default")]
        editable: bool,
    },
}

fn package_info_git_editable_default() -> bool {
    false
}

