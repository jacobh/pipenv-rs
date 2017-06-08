use serde_json;
use std::fmt;
use std::marker::PhantomData;
use serde::de::{Deserialize, Deserializer, Visitor, MapAccess};

type SerdeObject = serde_json::Map<String, serde_json::Value>;

#[derive(Deserialize, Debug)]
pub struct Package {
    info: SerdeObject,
    releases: PackageReleases,
}

#[derive(Debug)]
struct PackageReleases(Vec<PackageRelease>);
impl PackageReleases {
    fn new() -> PackageReleases {
        PackageReleases(vec![])
    }
}

#[derive(Debug)]
struct PackageRelease {
    version: String,
    release_variants: Vec<SerdeObject>,
}

// Reworked example from https://serde.rs/deserialize-map.html
struct PackageReleaseMapVisitor {
    marker: PhantomData<PackageReleases>,
}

impl PackageReleaseMapVisitor {
    fn new() -> Self {
        PackageReleaseMapVisitor { marker: PhantomData }
    }
}

impl<'de> Visitor<'de> for PackageReleaseMapVisitor {
    type Value = PackageReleases;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(r#"{"1.2.3": {...}, "2.0.0": {...}}"#)
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
        where M: MapAccess<'de>
    {
        let mut releases = PackageReleases::new();
        while let Some((key, value)) = access.next_entry()? {
            releases
                .0
                .push(PackageRelease {
                          version: key,
                          release_variants: value,
                      });
        }

        Ok(releases)
    }
}

impl<'de> Deserialize<'de> for PackageReleases {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        deserializer.deserialize_map(PackageReleaseMapVisitor::new())
    }
}
