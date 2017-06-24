use std::io;
use std::io::Read;
use flate2::read::GzDecoder;
use tar::Archive as TarArchive;
use zip::read::ZipArchive;
use serde_json;

use version_req::PackageVersionReq;
use release::{ReleaseType, WheelMetadata};
use errors::*;

fn get_wheel_metadata_from_archive_file<R>(mut file: R) -> Result<WheelMetadata>
where
    R: io::Read,
{
    let mut archive = {
        let mut bytes = vec![];
        file.read_to_end(&mut bytes)?;
        ZipArchive::new(io::Cursor::new(bytes))?
    };
    for i in 0..archive.len() {
        let file = archive.by_index(i)?;
        if file.name().ends_with(".dist-info/metadata.json") {
            println!("parsing {}", file.name());
            let wheel_meta: WheelMetadata = serde_json::from_reader(file)?;
            return Ok(wheel_meta);
        }
    }
    bail!(ErrorKind::ArchiveFileNotFound(
        ".dist-info/metadata.json".to_owned()
    ));
}

fn parse_requires_txt(text: &str) -> Result<Vec<PackageVersionReq>> {
    text.split("\n")
        .filter(|line| line != &"")
        .take_while(|line| !line.starts_with("["))
        .map(|line| PackageVersionReq::parse_requirement(line))
        .collect()
}

pub fn parse_release_requirements<R>(
    file: R,
    release_type: ReleaseType,
) -> Result<Vec<PackageVersionReq>>
where
    R: io::Read,
{
    match release_type {
        ReleaseType::BdistWheel => {
            let wheel_meta = get_wheel_metadata_from_archive_file(file)?;
            wheel_meta.to_version_reqs()
        }
        ReleaseType::Sdist => {
            let mut archive = TarArchive::new(GzDecoder::new(file)?);
            for entry in archive.entries()? {
                let mut entry = entry?;
                if entry
                    .path()?
                    .to_string_lossy()
                    .ends_with(".egg-info/requires.txt")
                {
                    let requires_txt = {
                        let mut data = String::new();
                        entry.read_to_string(&mut data)?;
                        data
                    };
                    return Ok(parse_requires_txt(&requires_txt)?);
                }
            }
            bail!(ErrorKind::ArchiveFileNotFound(
                ".egg-info/requires.txt".to_owned()
            ));
        }
        _ => unimplemented!(),
    }
}

#[cfg(test)]
mod tests {
    use semver;
    use parse_release::*;

    fn make_version_req(name: &str, reqs: Vec<&str>) -> PackageVersionReq {
        PackageVersionReq::new(
            name.to_owned(),
            reqs.iter()
                .map(|s| semver::VersionReq::parse(s).unwrap())
                .collect(),
        )

    }

    #[test]
    fn parse_requests_requires_txt() {
        let requires_txt = "chardet>=3.0.2,<3.1.0
idna>=2.5,<2.6
urllib3>=1.21.1,<1.22
certifi>=2017.4.17

[security]
pyOpenSSL>=0.14";

        let version_reqs = parse_requires_txt(requires_txt).unwrap();

        assert_eq!(
            version_reqs,
            vec![
                make_version_req("chardet", vec![">= 3.0.2", "< 3.1.0"]),
                make_version_req("idna", vec![">= 2.5", "< 2.6"]),
                make_version_req("urllib3", vec![">= 1.21.1", "< 1.22"]),
                make_version_req("certifi", vec![">= 2017.4.17"]),
            ]
        )
    }
}
