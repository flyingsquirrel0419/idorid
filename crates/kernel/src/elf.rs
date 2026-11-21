//! execve(): loads an ELF executable (and its interpreter) into a fresh
//! address space and builds the initial stack (argv, envp, auxv).

use crate::errno::*;
use crate::mm::*;
use crate::vfs::{FileRef, Result};
use std::sync::Arc;

pub struct Image {
    pub mm: Arc<Mm>,
    pub entry: u64,
    pub sp: u64,
}

struct Ehdr {
    etype: u16,
    entry: u64,
    phoff: u64,
    phentsize: u16,
    phnum: u16,
}

#[derive(Clone, Copy)]
struct Phdr {
    ptype: u32,
    flags: u32,
    offset: u64,
    vaddr: u64,
    filesz: u64,
    memsz: u64,
}

fn read_exact(f: &FileRef, off: u64, n: usize) -> Result<Vec<u8>> {
    let mut v = vec![0u8; n];
    let mut done = 0;
    while done < n {
        let r = f.read_at(off + done as u64, &mut v[done..], false)?;
        if r == 0 {
            return Err(ENOEXEC);
        }
        done += r;
    }
    Ok(v)
}

fn parse(f: &FileRef) -> Result<(Ehdr, Vec<Phdr>)> {
    let h = read_exact(f, 0, 64)?;
    if &h[..4] != b"\x7fELF" || h[4] != 2 || h[5] != 1 {
        return Err(ENOEXEC);
    }
    let u16_ = |o: usize| u16::from_le_bytes(h[o..o + 2].try_into().unwrap());
    let u64_ = |o: usize| u64::from_le_bytes(h[o..o + 8].try_into().unwrap());
    if u16_(18) != 183 {
        return Err(ENOEXEC);
    }
    let e = Ehdr {
        etype: u16_(16),
        entry: u64_(24),
        phoff: u64_(32),
        phentsize: u16_(54),
        phnum: u16_(56),
    };
    if e.phentsize as usize != 56 || e.phnum > 128 {
        return Err(ENOEXEC);
    }
    let ph = read_exact(f, e.phoff, 56 * e.phnum as usize)?;
    let mut out = Vec::new();
    for i in 0..e.phnum as usize {
        let b = &ph[i * 56..];
        let u32_ = |o: usize| u32::from_le_bytes(b[o..o + 4].try_into().unwrap());
        let u64_ = |o: usize| u64::from_le_bytes(b[o..o + 8].try_into().unwrap());
        out.push(Phdr {
            ptype: u32_(0),
            flags: u32_(4),
            offset: u64_(8),
            vaddr: u64_(16),
            filesz: u64_(32),
            memsz: u64_(40),
        });
    }
    Ok((e, out))
}

fn prot_of(flags: u32) -> u32 {
    (if flags & 4 != 0 { PROT_READ } else { 0 })
        | (if flags & 2 != 0 { PROT_WRITE } else { 0 })
        | (if flags & 1 != 0 { PROT_EXEC } else { 0 })
}

/// Maps the PT_LOAD segments; returns (load bias, phdr address).
fn map_segments(
    mm: &Mm,
    f: &FileRef,
    e: &Ehdr,
    ph: &[Phdr],
    name: &str,
    base_hint: u64,
) -> Result<(u64, u64)> {
    let loads: Vec<&Phdr> = ph.iter().filter(|p| p.ptype == 1).collect();
    if loads.is_empty() {
        return Err(ENOEXEC);
    }
    let lo = page_down(loads.iter().map(|p| p.vaddr).min().unwrap());
    let hi = page_up(loads.iter().map(|p| p.vaddr + p.memsz).max().unwrap());
    let bias = if e.etype == 3 {
        // ET_DYN: pick a free range.
        let _map_guard = mm.map_lock.lock();
        let at = mm.find_free(base_hint, hi - lo).ok_or(ENOMEM)?;
        at - lo
    } else {
        0
    };
    let name: Arc<str> = Arc::from(name);
    for p in &loads {
        let prot = prot_of(p.flags);
        let start = page_down(p.vaddr + bias);
        let file_end = p.vaddr + bias + p.filesz;
        let mem_end = page_up(p.vaddr + bias + p.memsz);
        let off = p.offset - (p.vaddr + bias - start);
        let file_map_end = page_up(file_end);
        if p.filesz > 0 {
            mm.map(
                start,
                file_map_end - start,
                prot,
                Backing::File {
                    file: f.clone(),
                    offset: off,
                },
                name.clone(),
            );
            // Zero the tail of the last file page (bss start).
            if file_end & PAGE_MASK != 0 && p.memsz > p.filesz {
                let n = (page_up(file_end) - file_end) as usize;
                mm.poke(file_end, &vec![0u8; n]);
            }
        }
        let anon_start = if p.filesz > 0 { file_map_end } else { start };
        if mem_end > anon_start {
            mm.map(
                anon_start,
                mem_end - anon_start,
                prot,
                Backing::Anon,
                Arc::from("[anon:.bss]"),
            );
        }
    }
    // Program headers' address.
    let phdr = ph
        .iter()
        .find(|p| p.ptype == 6)
        .map(|p| p.vaddr + bias)
        .or_else(|| {
            loads
                .iter()
                .find(|p| p.offset <= e.phoff && e.phoff < p.offset + p.filesz)
                .map(|p| p.vaddr + bias + (e.phoff - p.offset))
        })
        .unwrap_or(0);
    Ok((bias, phdr))
}

pub struct ExecArgs<'a> {
    pub file: FileRef,
    pub path: &'a str,
    pub argv: Vec<Vec<u8>>,
    pub envp: Vec<Vec<u8>>,
    /// Opens the interpreter named by PT_INTERP.
    pub open_interp: &'a dyn Fn(&str) -> Result<FileRef>,
    pub hwcap: u64,
    pub hwcap2: u64,
    pub uid: u32,
    pub gid: u32,
}

pub const SIGRETURN_TRAMPOLINE: u64 = 0x7f_ffff_0000;
pub const STACK_TOP: u64 = 0x7f_f000_0000;
pub const STACK_SIZE: u64 = 8 << 20;

pub fn load(a: ExecArgs) -> Result<Image> {
    // Scripts: "#!interp [arg]"
    let head = {
        let mut h = vec![0u8; 256];
        let n = a.file.read_at(0, &mut h, false)?;
        h.truncate(n);
        h
    };
    if head.starts_with(b"#!") {
        let line_end = head.iter().position(|&c| c == b'\n').unwrap_or(head.len());
        let line = String::from_utf8_lossy(&head[2..line_end])
            .trim()
            .to_string();
        let mut parts = line.splitn(2, |c: char| c.is_ascii_whitespace());
        let interp = parts.next().ok_or(ENOEXEC)?.to_string();
        let arg = parts.next().map(|s| s.trim().to_string());
        let mut argv = vec![interp.clone().into_bytes()];
        if let Some(x) = arg {
            argv.push(x.into_bytes());
        }
        argv.push(a.path.as_bytes().to_vec());
        argv.extend(a.argv.into_iter().skip(1));
        let f = (a.open_interp)(&interp)?;
        return load(ExecArgs {
            file: f,
            path: &interp,
            argv,
            envp: a.envp,
            open_interp: a.open_interp,
            hwcap: a.hwcap,
            hwcap2: a.hwcap2,
            uid: a.uid,
            gid: a.gid,
        });
    }
    let (e, ph) = parse(&a.file)?;
    let mm = Arc::new(Mm::new());
    let (bias, phdr) = map_segments(&mm, &a.file, &e, &ph, a.path, 0x55_5555_0000)?;
    let entry = e.entry + bias;
    let mut interp_base = 0;
    let mut start = entry;
    if let Some(ip) = ph.iter().find(|p| p.ptype == 3) {
        let raw = read_exact(&a.file, ip.offset, ip.filesz as usize)?;
        let name = String::from_utf8_lossy(raw.split(|&b| b == 0).next().unwrap()).into_owned();
        let f = (a.open_interp)(&name)?;
        let (ie, iph) = parse(&f)?;
        let (ibias, _) = map_segments(&mm, &f, &ie, &iph, &name, 0x7f_c000_0000)?;
        interp_base = ibias;
        start = ie.entry + ibias;
    }
    // brk starts after the executable's highest segment.
    let end = ph
        .iter()
        .filter(|p| p.ptype == 1)
        .map(|p| page_up(p.vaddr + bias + p.memsz))
        .max()
        .unwrap();
    *mm.brk.lock() = (end, end);

    // Signal return trampoline (what the vDSO provides on Linux; used when a
    // handler is installed without SA_RESTORER, e.g. by glibc).
    mm.map(
        SIGRETURN_TRAMPOLINE,
        PAGE,
        PROT_READ | PROT_EXEC,
        Backing::Anon,
        Arc::from("[vdso]"),
    );
    // mov x8, #139 ; svc #0
    mm.poke(
        SIGRETURN_TRAMPOLINE,
        &[0x68, 0x11, 0x80, 0xd2, 0x01, 0x00, 0x00, 0xd4],
    );
    // Stack.
    mm.map(
        STACK_TOP - STACK_SIZE,
        STACK_SIZE,
        PROT_READ | PROT_WRITE,
        Backing::Anon,
        Arc::from("[stack]"),
    );
    let mut sp = STACK_TOP;
    let mut push = |data: &[u8]| -> u64 {
        sp -= data.len() as u64;
        mm.poke(sp, data);
        sp
    };
    let execfn = push(&[a.path.as_bytes(), b"\0"].concat());
    let mut envp_ptrs = Vec::new();
    for s in a.envp.iter().rev() {
        envp_ptrs.push(push(&[s.as_slice(), b"\0"].concat()));
    }
    envp_ptrs.reverse();
    let mut argv_ptrs = Vec::new();
    for s in a.argv.iter().rev() {
        argv_ptrs.push(push(&[s.as_slice(), b"\0"].concat()));
    }
    argv_ptrs.reverse();
    let platform = push(b"aarch64\0");
    let mut rnd = [0u8; 16];
    crate::fd::fill_random(&mut rnd);
    let random = push(&rnd);
    let auxv: Vec<(u64, u64)> = vec![
        (33, 0), // AT_SYSINFO_EHDR: no vDSO
        (16, a.hwcap),
        (6, 4096),
        (17, 100),
        (3, phdr),
        (4, 56),
        (5, e.phnum as u64),
        (7, interp_base),
        (8, 0),
        (9, entry),
        (11, a.uid as u64),
        (12, a.uid as u64),
        (13, a.gid as u64),
        (14, a.gid as u64),
        (23, 0),
        (25, random),
        (26, a.hwcap2),
        (31, execfn),
        (15, platform),
        (51, 5120), // AT_MINSIGSTKSZ
        (0, 0),
    ];
    let mut words: Vec<u64> = Vec::new();
    words.push(a.argv.len() as u64);
    words.extend(&argv_ptrs);
    words.push(0);
    words.extend(&envp_ptrs);
    words.push(0);
    for (k, v) in auxv {
        if k == 33 {
            continue;
        }
        words.push(k);
        words.push(v);
    }
    let bytes: Vec<u8> = words.iter().flat_map(|w| w.to_le_bytes()).collect();
    let sp = (sp - bytes.len() as u64) & !15;
    mm.poke(sp, &bytes);
    Ok(Image {
        mm,
        entry: start,
        sp,
    })
}
