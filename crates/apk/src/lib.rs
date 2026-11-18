#![forbid(unsafe_code)]

mod android_attrs;
pub mod axml;
mod binary;
pub mod resources;
pub use resources::{Bag, ResValue, Resources, DEVICE_SDK};

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

/// An APK opened lazily: entries are decompressed on demand.
pub struct Apk {
    zip: std::cell::RefCell<ZipArchive<Cursor<Vec<u8>>>>,
    names: Vec<String>,
    index: std::collections::HashMap<String, usize>,
}
impl Apk {
    pub fn open(bytes: Vec<u8>) -> Result<Self, ApkError> {
        let zip = ZipArchive::new(Cursor::new(bytes))?;
        let names: Vec<String> = (0..zip.len())
            .map(|i| zip.name_for_index(i).unwrap_or_default().to_owned())
            .collect();
        let index = names
            .iter()
            .enumerate()
            .map(|(i, n)| (n.clone(), i))
            .collect();
        Ok(Self {
            zip: std::cell::RefCell::new(zip),
            names,
            index,
        })
    }
    pub fn entries(&self) -> Vec<&str> {
        self.names.iter().map(|n| n.as_str()).collect()
    }
    pub fn read(&self, name: &str) -> Option<Vec<u8>> {
        let i = *self.index.get(name)?;
        let mut zip = self.zip.borrow_mut();
        let mut file = zip.by_index(i).ok()?;
        let mut data = Vec::with_capacity(file.size() as usize);
        file.read_to_end(&mut data).ok()?;
        Some(data)
    }
    pub fn dex_files(&self) -> Vec<Vec<u8>> {
        let mut dex: Vec<(u32, &str)> = self
            .names
            .iter()
            .filter_map(|n| {
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
                Some((index, n.as_str()))
            })
            .collect();
        dex.sort_by_key(|(i, _)| *i);
        dex.into_iter().filter_map(|(_, n)| self.read(n)).collect()
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
