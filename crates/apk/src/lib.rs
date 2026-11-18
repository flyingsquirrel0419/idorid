#![forbid(unsafe_code)]

mod android_attrs;
pub mod axml;
mod binary;
pub mod resources;
pub use resources::{ResValue, Resources};

use std::{
    fmt,
    io::{Cursor, Read},
};
use zip::ZipArchive;

#[derive(Debug)]
pub struct ApkError(pub String);
impl fmt::Display for ApkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
impl std::error::Error for ApkError {}
impl From<zip::result::ZipError> for ApkError {
    fn from(e: zip::result::ZipError) -> Self {
        Self(e.to_string())
    }
}
impl From<std::io::Error> for ApkError {
    fn from(e: std::io::Error) -> Self {
        Self(e.to_string())
    }
}

pub struct Apk {
    entries: Vec<(String, Vec<u8>)>,
}
impl Apk {
    pub fn open(bytes: Vec<u8>) -> Result<Self, ApkError> {
        let mut zip = ZipArchive::new(Cursor::new(bytes))?;
        let mut entries = Vec::new();
        for i in 0..zip.len() {
            let mut file = zip.by_index(i)?;
            let name = file.name().to_owned();
            let mut data = Vec::new();
            file.read_to_end(&mut data)?;
            entries.push((name, data));
        }
        Ok(Self { entries })
    }
    pub fn entries(&self) -> Vec<&str> {
        self.entries.iter().map(|(n, _)| n.as_str()).collect()
    }
    pub fn read(&self, name: &str) -> Option<Vec<u8>> {
        self.entries
            .iter()
            .find(|(n, _)| n == name)
            .map(|(_, b)| b.clone())
    }
    pub fn dex_files(&self) -> Vec<Vec<u8>> {
        let mut dex: Vec<_> = self
            .entries
            .iter()
            .filter_map(|(n, b)| {
                let suffix = n.strip_prefix("classes")?.strip_suffix(".dex")?;
                let index = if suffix.is_empty() {
                    1
                } else {
                    let i: u32 = suffix.parse().ok()?;
                    if i < 2 || suffix.starts_with('0') {
                        return None;
                    }
                    i
                };
                Some((index, b))
            })
            .collect();
        dex.sort_by_key(|(i, _)| *i);
        dex.into_iter().map(|(_, b)| b.clone()).collect()
    }
    pub fn manifest(&self) -> Result<Manifest, ApkError> {
        Manifest::parse(
            &self
                .read("AndroidManifest.xml")
                .ok_or_else(|| ApkError("missing AndroidManifest.xml".into()))?,
        )
    }
    pub fn resources(&self) -> Result<Resources, ApkError> {
        Resources::parse(
            &self
                .read("resources.arsc")
                .ok_or_else(|| ApkError("missing resources.arsc".into()))?,
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Manifest {
    pub package: String,
    pub version_name: Option<String>,
    pub label: Option<ResValue>,
    pub main_activity: Option<String>,
    pub min_sdk: Option<u32>,
    pub application_class: Option<String>,
}
impl Manifest {
    pub fn parse(data: &[u8]) -> Result<Self, ApkError> {
        use axml::Element;
        fn attr<'a>(e: &'a Element, name: &str) -> Option<&'a ResValue> {
            e.attributes
                .iter()
                .find(|a| a.name == name)
                .map(|a| &a.value)
        }
        fn string(e: &Element, name: &str) -> Option<String> {
            match attr(e, name)? {
                ResValue::String(s) => Some(s.clone()),
                _ => None,
            }
        }
        fn qualify(pkg: &str, name: String) -> String {
            if name.starts_with('.') {
                format!("{pkg}{name}")
            } else if !name.contains('.') {
                format!("{pkg}.{name}")
            } else {
                name
            }
        }
        let doc = axml::Document::parse(data)?;
        let root = &doc.root;
        let package =
            string(root, "package").ok_or_else(|| ApkError("manifest missing package".into()))?;
        let app = root.children.iter().find(|c| c.name == "application");
        let mut result = Self {
            package: package.clone(),
            version_name: string(root, "versionName"),
            label: app.and_then(|a| attr(a, "label").cloned()),
            main_activity: None,
            min_sdk: root
                .children
                .iter()
                .find(|c| c.name == "uses-sdk")
                .and_then(|c| match attr(c, "minSdkVersion") {
                    Some(ResValue::Int(i)) => Some(*i as u32),
                    _ => None,
                }),
            application_class: app
                .and_then(|a| string(a, "name"))
                .map(|s| qualify(&package, s)),
        };
        if let Some(app) = app {
            for activity in &app.children {
                if activity.name != "activity" && activity.name != "activity-alias" {
                    continue;
                }
                let launcher = activity
                    .children
                    .iter()
                    .filter(|e| e.name == "intent-filter")
                    .any(|filter| {
                        let has = |tag: &str, value: &str| {
                            filter.children.iter().any(|e| {
                                e.name == tag && string(e, "name").as_deref() == Some(value)
                            })
                        };
                        has("action", "android.intent.action.MAIN")
                            && has("category", "android.intent.category.LAUNCHER")
                    });
                if launcher {
                    result.main_activity = string(
                        activity,
                        if activity.name == "activity-alias" {
                            "targetActivity"
                        } else {
                            "name"
                        },
                    )
                    .map(|s| qualify(&package, s));
                    break;
                }
            }
        }
        Ok(result)
    }
}
