use regex::Regex;
use std::io::{Read, Cursor};
use flate2::read::GzDecoder;
use tar::Archive as TarArchive;
use zip::read::ZipArchive;
use semver;
use serde_json;

use version_req::PackageVersionReq;
use release::{ReleaseType, WheelMetadata};

fn parse_requires_txt_line(line: &str) -> Option<PackageVersionReq> {
    lazy_static! {
        static ref PACKAGE_NAME_RE: Regex = Regex::new(r"\w+").unwrap();
        static ref VERSION_REQ_RE: Regex = Regex::new(r"[<=>]{1,2}\d+(\.\d+){0,2}").unwrap();
    }
    let package_name = PACKAGE_NAME_RE.find(line).unwrap().as_str().to_owned();
    let version_reqs = VERSION_REQ_RE
        .find_iter(line)
        .filter_map(|x| semver::VersionReq::parse(x.as_str()).ok())
        .collect();
    Some(PackageVersionReq::new(package_name, version_reqs))
}

fn parse_requires_txt(text: &str) -> Vec<PackageVersionReq> {
    text.split("\n")
        .filter(|line| line != &"")
        .take_while(|line| !line.starts_with("["))
        .filter_map(|line| parse_requires_txt_line(line))
        .collect()
}

pub fn parse_release_requirements<R>(mut file: R,
                                     release_type: ReleaseType)
                                     -> Option<Vec<PackageVersionReq>>
    where R: Read
{
    match release_type {
        ReleaseType::BdistWheel => {
            let mut archive = {
                let mut bytes = vec![];
                file.read_to_end(&mut bytes).unwrap();
                ZipArchive::new(Cursor::new(bytes)).unwrap()
            };
            for i in 0..archive.len() {
                let file = archive.by_index(i).unwrap();
                if file.name().ends_with(".dist-info/metadata.json") {
                    println!("parsing {}", file.name());
                    let wheel_meta: WheelMetadata = serde_json::from_reader(file).unwrap();
                }
            }
            None
        }
        ReleaseType::Sdist => {
            let mut archive = TarArchive::new(GzDecoder::new(file).unwrap());
            for entry in archive.entries().unwrap() {
                let mut entry = entry.unwrap();
                if entry
                       .path()
                       .unwrap()
                       .to_string_lossy()
                       .ends_with(".egg-info/requires.txt") {
                    let requires_txt = {
                        let mut data = String::new();
                        entry
                            .read_to_string(&mut data)
                            .expect("failed to read requires.txt");
                        data
                    };
                    return Some(parse_requires_txt(&requires_txt));
                }
            }
            None
        }
        _ => unimplemented!(),
    }
}

#[cfg(test)]
mod tests {
    use parse_release::*;

    fn make_version_req(name: &str, reqs: Vec<&str>) -> PackageVersionReq {
        PackageVersionReq::new(name.to_owned(),
                               reqs.iter()
                                   .map(|s| semver::VersionReq::parse(s).unwrap())
                                   .collect())

    }

    #[test]
    fn parse_simple_requires_txt_line() {
        let requires_txt_line = "chardet<3.1.0";

        let version_req = parse_requires_txt_line(requires_txt_line);

        assert_eq!(version_req,
                   Some(make_version_req("chardet", vec!["< 3.1.0"])));
    }

    #[test]
    fn parse_requires_txt_range() {
        let requires_txt_line = "chardet>=3.0.2,<3.1.0";

        let version_req = parse_requires_txt_line(requires_txt_line);

        assert_eq!(version_req,
                   Some(make_version_req("chardet", vec![">= 3.0.2", "< 3.1.0"])));
    }

    #[test]
    fn parse_major_only() {
        let requires_txt_line = "django<2";

        let version_req = parse_requires_txt_line(requires_txt_line);

        assert_eq!(version_req, Some(make_version_req("django", vec!["< 2"])));
    }

    #[test]
    fn parse_requests_requires_txt() {
        let requires_txt = "chardet>=3.0.2,<3.1.0
idna>=2.5,<2.6
urllib3>=1.21.1,<1.22
certifi>=2017.4.17

[security]
pyOpenSSL>=0.14";

        let version_reqs = parse_requires_txt(requires_txt);

        assert_eq!(version_reqs,
                   vec![make_version_req("chardet", vec![">= 3.0.2", "< 3.1.0"]),
                        make_version_req("idna", vec![">= 2.5", "< 2.6"]),
                        make_version_req("urllib3", vec![">= 1.21.1", "< 1.22"]),
                        make_version_req("certifi", vec![">= 2017.4.17"])])
    }
}

