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

