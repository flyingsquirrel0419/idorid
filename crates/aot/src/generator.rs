use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::{Component, Path};

/// Generate Rust units from the profiler's hottest file-backed pages.
pub fn generate(
    profile: &Path,
    rom: &Path,
    limit: usize,
    mut out: impl Write,
) -> io::Result<usize> {
    let text = std::fs::read_to_string(profile)?;
    writeln!(out, "use idorid_a64::{{Cpu, Exit, AddressSpace}};")?;
    let mut pages = Vec::new();
    for line in text.lines().filter(|l| !l.starts_with('#')) {
        if pages.len() >= limit {
            break;
        }
        let fields: Vec<_> = line.split_whitespace().collect();
        if fields.len() != 3 || fields[1].starts_with('[') {
            continue;
        }
        let path = Path::new(fields[1].trim_start_matches('/'));
        if path
            .components()
            .any(|x| !matches!(x, Component::Normal(_)))
        {
            continue;
        }
        let offset = match u64::from_str_radix(fields[2].trim_start_matches("0x"), 16) {
            Ok(v) if v % 4096 == 0 => v,
            _ => continue,
        };
        if pages
            .iter()
            .any(|(p, o, _, _)| p == fields[1] && *o == offset)
        {
            continue;
        }
        let mut image = match std::fs::File::open(rom.join(path)) {
            Ok(v) => v,
            Err(_) => continue,
        };
        let mut page = [0u8; 4096];
        if image
            .seek(SeekFrom::Start(offset))
            .and_then(|_| image.read_exact(&mut page))
            .is_err()
        {
            continue;
        }
        let bytes = &page[..];
        let mut hash = 0xcbf29ce484222325u64;
        for b in bytes {
            hash = (hash ^ *b as u64).wrapping_mul(0x100000001b3);
        }
        let n = pages.len();
        writeln!(out, "static BYTES_{n}: [u8; 4096] = {:?};", bytes)?;
        writeln!(
            out,
            "fn unit_{n}(cpu: &mut Cpu, mem: &dyn AddressSpace, budget: u64) -> Option<Exit> {{"
        )?;
        writeln!(out,"let base = cpu.pc & !4095; let end = cpu.retired.saturating_add(budget); let generation = mem.generation();")?;
        writeln!(out,"while cpu.retired < end && cpu.pc & !4095 == base && cpu.pc & 3 == 0 && mem.generation() == generation {{")?;
        writeln!(
            out,
            "let entry = ((cpu.pc & 4095) >> 2) as usize; match entry / 8 {{"
        )?;
        for (group, chunk) in bytes.as_chunks::<32>().0.iter().enumerate() {
            writeln!(out, "{group} => {{")?;
            for (j, word) in chunk.as_chunks::<4>().0.iter().enumerate() {
                let index = group * 8 + j;
                let word = u32::from_le_bytes(*word);
                let next = (index + 1) * 4;
                let aes = matches!(
                    word & 0xffff_fc00,
                    0x4e28_4800 | 0x4e28_5800 | 0x4e28_6800 | 0x4e28_7800
                );
                let boolean = matches!(word & 0xbfe0_fc00, 0x2e20_1c00 | 0x0ea0_1c00);
                if aes || boolean {
                    writeln!(out,"if entry <= {index} {{ idorid_a64::aot::aot_simd_simple(cpu,0x{word:08x}); if cpu.pc != base.wrapping_add({next}) || cpu.retired >= end {{ continue; }} }}")?;
                } else {
                    writeln!(out,"if entry <= {index} {{ if let Some(e)=idorid_a64::aot::aot_step(cpu,mem,0x{word:08x}) {{ return Some(e); }} if cpu.pc != base.wrapping_add({next}) || cpu.retired >= end || mem.generation() != generation {{ continue; }} }}")?;
                }
            }
            writeln!(out, "}},")?;
        }
        writeln!(out, "_ => return None, }} }}")?;
        writeln!(
            out,
            "if cpu.retired >= end {{ Some(Exit::Budget) }} else {{ None }} }}"
        )?;
        pages.push((fields[1].to_string(), offset, hash, n));
    }
    writeln!(out, "pub const HAS_UNITS: bool = {};", !pages.is_empty())?;
    writeln!(out,"pub fn lookup(path: &str, offset: u64) -> Option<idorid_a64::mem::AotUnit> {{ match (path, offset) {{")?;
    for (path, offset, hash, n) in &pages {
        writeln!(out,"({path:?}, {offset}) => Some(idorid_a64::mem::AotUnit {{ fingerprint: {hash}, bytes: &BYTES_{n}, execute: unit_{n} }}),")?;
    }
    writeln!(out, "_ => None, }} }}")?;
    Ok(pages.len())
}
