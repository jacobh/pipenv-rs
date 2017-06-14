use semver;

error_chain!{
    foreign_links {
        SerdeJson(::serde_json::Error);
        Io(::std::io::Error);
        SemVerReqParse(::semver::ReqParseError);
        SemVer(::semver::SemVerError);
        Zip(::zip::result::ZipError);
        Reqwest(::reqwest::Error);
    }
    errors {
            PackageNameRegexFailed(s: String) {
                description("Package name regex failed")
                display("Package name regex failed: `{}`", s)
            }
            NormalizeVersionStringRegexFailed(s: String) {
                description("Normalize version string regex failed")
                display("Normalize version string regex failed: `{}`", s)
            }
            ArchiveFileNotFound(s: String) {
                description("File not found in archive")
                display("File not found in archive: `{}`", s)
            }
            VersionDoesntExist(name: String, v: semver::Version) {
                description("Version doesn't exist")
                display("Version doesn't exist: {}: {}", name, v)
            }
            NoReleaseForVersion(name: String, v: semver::Version) {
                description("No release found for version")
                display("No release found for version: {}: {}", name, v)
            }
            PackageHasNoReleasedVersions(s: String) {
                description("Package has no released versions")
                display("Package has no released versions: {}", s)
            }
    }
}

