use regex;
use regex::Regex;
use semver;

use errors::*;

fn match_to_int(capture: Option<regex::Match>) -> u16 {
    match capture {
        Some(match_) => {
            let match_str = match_.as_str().trim_left_matches("0");
            match_str.parse().unwrap_or(0)
        }
        None => 0,
    }
}

fn normalize_version_string(version: &str) -> Result<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\d+).?(\d+)?.?(\d+)?(.*)$").unwrap();
    }
    let captures = RE.captures(version).ok_or_else(|| {
        ErrorKind::NormalizeVersionStringRegexFailed(version.to_owned())
    })?;

    let maj: u16 = match_to_int(captures.get(1));
    let min: u16 = match_to_int(captures.get(2));
    let patch: u16 = match_to_int(captures.get(3));

    Ok(format!("{}.{}.{}", maj, min, patch))
}

pub fn normalize_and_parse_version_string(version: &str) -> Result<semver::Version> {
    match semver::Version::parse(version) {
        Ok(version) => Ok(version),
        Err(_) => semver::Version::parse(&normalize_version_string(version)?).map_err(|e| e.into()),
    }
}

#[cfg(test)]
mod tests {
    use semver_utils::*;

    #[test]
    fn fix_leading_zero() {
        let version = "1.01.0";

        let fixed = normalize_version_string(version).unwrap();

        assert_eq!(fixed, "1.1.0");
    }

    #[test]
    fn fix_missing_patch() {
        let version = "3.2";

        let fixed = normalize_version_string(version).unwrap();

        assert_eq!(fixed, "3.2.0");
    }

    #[test]
    fn fix_missing_minor() {
        let version = "5";

        let fixed = normalize_version_string(version).unwrap();

        assert_eq!(fixed, "5.0.0");
    }
}
