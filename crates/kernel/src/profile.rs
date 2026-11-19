//! Statistical profile of where guest time goes, by (mapped file, 4 KiB page
//! offset). Used to choose which ROM code pages to translate ahead of time.

use crate::mm::{Backing, Mm};
use parking_lot::Mutex;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct Profile {
    out: PathBuf,
    samples: Mutex<HashMap<(String, u64), u64>>,
    total: Mutex<u64>,
}

impl Profile {
    pub fn new(out: PathBuf) -> Profile {
        Profile {
            out,
            samples: Mutex::new(HashMap::new()),
            total: Mutex::new(0),
        }
    }

    pub fn sample(&self, mm: &Mm, pc: u64) {
        let key = match mm.vma_at(pc) {
            Some(v) => match &v.backing {
                Backing::File { offset, .. } => (v.name.to_string(), (offset + (pc - v.start)) & !0xfff),
                _ => (format!("[anon:{}]", v.name), (pc - v.start) & !0xfff),
            },
            None => ("[unmapped]".into(), 0),
        };
        *self.samples.lock().entry(key).or_default() += 1;
        *self.total.lock() += 1;
    }

    /// Writes "samples file page_offset" lines, hottest first.
    pub fn dump(&self) {
        let s = self.samples.lock();
        let mut v: Vec<_> = s.iter().collect();
        v.sort_by(|a, b| b.1.cmp(a.1));
        let total = *self.total.lock();
        let mut out = format!("# {} samples, {} pages\n", total, v.len());
        for ((file, page), n) in v {
            out.push_str(&format!("{n} {file} {page:#x}\n"));
        }
        let _ = std::fs::write(&self.out, out);
    }
}

impl Drop for Profile {
    fn drop(&mut self) {
        self.dump();
    }
}
