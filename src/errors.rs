error_chain!{
    foreign_links {
        SerdeJson(::serde_json::Error);
        Io(::std::io::Error);
        SemVerReqParse(::semver::ReqParseError);
        Zip(::zip::result::ZipError);
    }
    errors {
            PackageNameRegexFailed(s: String) {
                description("Package name regex failed")
                display("Package name regex failed: `{}`", s)
            }
            ArchiveFileNotFound(s: String) {
                description("File not found in archive")
                display("File not found in archive: `{}`", s)
            }
    }
}

