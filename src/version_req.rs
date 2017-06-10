use std::fmt;
use semver;

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

