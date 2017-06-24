use regex::Regex;
use std::fmt;
use semver;

use errors::*;

#[derive(PartialEq)]
pub struct PackageVersionReq {
    package_name: String,
    version_reqs: Vec<semver::VersionReq>,
}
impl PackageVersionReq {
    pub fn new(package_name: String, version_reqs: Vec<semver::VersionReq>) -> PackageVersionReq {
        PackageVersionReq {
            package_name: package_name,
            version_reqs: version_reqs,
        }
    }
    pub fn parse_requires_txt_line(line: &str) -> Result<PackageVersionReq> {
        lazy_static! {
            static ref PACKAGE_NAME_RE: Regex = Regex::new(r"\w+").unwrap();
            static ref VERSION_REQ_RE: Regex = Regex::new(r"[<=>]{1,2}\d+(\.\d+){0,2}").unwrap();
        }
        let package_name = PACKAGE_NAME_RE
            .find(line)
            .ok_or_else(|| ErrorKind::PackageNameRegexFailed(line.to_owned()))?
            .as_str()
            .to_owned();
        let version_reqs: Result<Vec<semver::VersionReq>> = VERSION_REQ_RE
            .find_iter(line)
            .map(|x| semver::VersionReq::parse(x.as_str()).map_err(|e| e.into()))
            .collect();
        Ok(Self::new(package_name, version_reqs?))
    }
}
impl fmt::Debug for PackageVersionReq {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{} ({})",
               self.package_name,
               self.version_reqs
                   .iter()
                   .map(|x| x.to_string())
                   .collect::<Vec<String>>()
                   .join(", "))
    }
}

#[cfg(test)]
mod tests {
    use version_req::*;

    fn make_version_req(name: &str, reqs: Vec<&str>) -> PackageVersionReq {
        PackageVersionReq::new(name.to_owned(),
                               reqs.iter()
                                   .map(|s| semver::VersionReq::parse(s).unwrap())
                                   .collect())

    }

    #[test]
    fn parse_simple_requires_txt_line() {
        let requires_txt_line = "chardet<3.1.0";

        let version_req = PackageVersionReq::parse_requires_txt_line(requires_txt_line).unwrap();

        assert_eq!(version_req, make_version_req("chardet", vec!["< 3.1.0"]));
    }

    #[test]
    fn parse_requires_txt_range() {
        let requires_txt_line = "chardet>=3.0.2,<3.1.0";

        let version_req = PackageVersionReq::parse_requires_txt_line(requires_txt_line).unwrap();

        assert_eq!(version_req,
                   make_version_req("chardet", vec![">= 3.0.2", "< 3.1.0"]));
    }

    #[test]
    fn parse_major_only() {
        let requires_txt_line = "django<2";

        let version_req = PackageVersionReq::parse_requires_txt_line(requires_txt_line).unwrap();

        assert_eq!(version_req, make_version_req("django", vec!["< 2"]));
    }
}

