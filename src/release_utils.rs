use reqwest;
use flate2::read::GzDecoder;
use tar::Archive;

pub fn get_tar_archive(url: &str) -> Archive<GzDecoder<reqwest::Response>> {
    Archive::new(GzDecoder::new(reqwest::get(url).unwrap()).unwrap())
}

