//! Read-only ext4 reader (enough for Android system images): extents and
//! legacy block maps, linear/htree directories (read linearly), fast and slow
//! symlinks, inline data, and extended attributes (in-inode and block).

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};

pub struct Ext4<R> {
    dev: R,
    /// Byte offset of the file system inside `dev`.
    base: u64,
    block_size: u64,
    inodes_per_group: u32,
    inode_size: u64,
    desc_size: u64,
    gdt_block: u64,
    is64: bool,
    groups: u32,
}

#[derive(Clone, Debug)]
pub struct Inode {
    pub ino: u32,
    pub mode: u16,
    pub uid: u32,
    pub gid: u32,
    pub size: u64,
    pub flags: u32,
    pub block: [u8; 60],
    pub file_acl: u64,
    pub extra: Vec<u8>, // bytes after the 128-byte base (in-inode xattrs)
}

pub const S_IFMT: u16 = 0o170000;
pub const S_IFDIR: u16 = 0o040000;
pub const S_IFREG: u16 = 0o100000;
pub const S_IFLNK: u16 = 0o120000;

const EXTENTS_FL: u32 = 0x80000;
const INLINE_DATA_FL: u32 = 0x1000_0000;

fn u16le(b: &[u8], o: usize) -> u16 {
    u16::from_le_bytes([b[o], b[o + 1]])
}
fn u32le(b: &[u8], o: usize) -> u32 {
    u32::from_le_bytes(b[o..o + 4].try_into().unwrap())
}

fn invalid(msg: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, msg.to_string())
}

impl<R: Read + Seek> Ext4<R> {
    pub fn open(mut dev: R, base: u64) -> io::Result<Ext4<R>> {
        let mut sb = [0u8; 1024];
        dev.seek(SeekFrom::Start(base + 1024))?;
        dev.read_exact(&mut sb)?;
        if u16le(&sb, 0x38) != 0xEF53 {
            return Err(invalid("not ext4"));
        }
        let block_size = 1024u64 << u32le(&sb, 24);
        let blocks = u32le(&sb, 4) as u64 | (u32le(&sb, 0x150) as u64) << 32;
        let blocks_per_group = u32le(&sb, 32) as u64;
        let first_data = u32le(&sb, 20) as u64;
        let incompat = u32le(&sb, 0x60);
        let is64 = incompat & 0x80 != 0;
        let desc_size = if is64 {
            u16le(&sb, 0xfe).max(32) as u64
        } else {
            32
        };
        let groups = (blocks - first_data).div_ceil(blocks_per_group) as u32;
        Ok(Ext4 {
            dev,
            base,
            block_size,
            inodes_per_group: u32le(&sb, 40),
            inode_size: u16le(&sb, 88) as u64,
            desc_size,
            gdt_block: first_data + 1,
            is64,
            groups,
        })
    }

    fn read_at(&mut self, off: u64, buf: &mut [u8]) -> io::Result<()> {
        self.dev.seek(SeekFrom::Start(self.base + off))?;
        self.dev.read_exact(buf)
    }

    fn read_block(&mut self, blk: u64) -> io::Result<Vec<u8>> {
        let mut b = vec![0u8; self.block_size as usize];
        self.read_at(blk * self.block_size, &mut b)?;
        Ok(b)
    }

    pub fn inode(&mut self, ino: u32) -> io::Result<Inode> {
        let group = (ino - 1) / self.inodes_per_group;
        if group >= self.groups {
            return Err(invalid("inode out of range"));
        }
        let idx = (ino - 1) % self.inodes_per_group;
        let mut d = vec![0u8; self.desc_size as usize];
        self.read_at(
            self.gdt_block * self.block_size + group as u64 * self.desc_size,
            &mut d,
        )?;
        let mut table = u32le(&d, 8) as u64;
        if self.is64 && d.len() >= 0x2c {
            table |= (u32le(&d, 0x28) as u64) << 32;
        }
        let mut raw = vec![0u8; self.inode_size as usize];
        self.read_at(
            table * self.block_size + idx as u64 * self.inode_size,
            &mut raw,
        )?;
        let mut block = [0u8; 60];
        block.copy_from_slice(&raw[40..100]);
        Ok(Inode {
            ino,
            mode: u16le(&raw, 0),
            uid: u16le(&raw, 2) as u32 | (u16le(&raw, 120) as u32) << 16,
            gid: u16le(&raw, 24) as u32 | (u16le(&raw, 122) as u32) << 16,
            size: u32le(&raw, 4) as u64 | (u32le(&raw, 108) as u64) << 32,
            flags: u32le(&raw, 32),
            block,
            file_acl: u32le(&raw, 104) as u64 | (u16le(&raw, 118) as u64) << 32,
            extra: if raw.len() > 128 {
                raw[128..].to_vec()
            } else {
                Vec::new()
            },
        })
    }

    /// Physical extents (logical block, physical block, count) of an inode.
    fn extents(&mut self, ino: &Inode) -> io::Result<Vec<(u64, u64, u64)>> {
        let mut out = Vec::new();
        if ino.flags & EXTENTS_FL != 0 {
            self.walk_extent_node(&ino.block, &mut out, 0)?;
        } else {
            // Legacy block map: 12 direct, then single/double/triple indirect.
            let ptrs: Vec<u32> = (0..15).map(|i| u32le(&ino.block, i * 4)).collect();
            let per = (self.block_size / 4) as u64;
            let mut logical = 0u64;
            for (i, &p) in ptrs.iter().enumerate() {
                let depth = if i < 12 { 0 } else { i - 11 };
                self.walk_indirect(p as u64, depth, &mut logical, per, &mut out)?;
            }
        }
        Ok(out)
    }

    fn walk_indirect(
        &mut self,
        blk: u64,
        depth: usize,
        logical: &mut u64,
        per: u64,
        out: &mut Vec<(u64, u64, u64)>,
    ) -> io::Result<()> {
        if depth == 0 {
            if blk != 0 {
                out.push((*logical, blk, 1));
            }
            *logical += 1;
            return Ok(());
        }
        if blk == 0 {
            *logical += per.pow(depth as u32);
            return Ok(());
        }
        let b = self.read_block(blk)?;
        for i in 0..per as usize {
            self.walk_indirect(u32le(&b, i * 4) as u64, depth - 1, logical, per, out)?;
        }
        Ok(())
    }

    fn walk_extent_node(
        &mut self,
        node: &[u8],
        out: &mut Vec<(u64, u64, u64)>,
        level: u32,
    ) -> io::Result<()> {
        if u16le(node, 0) != 0xF30A || level > 8 {
            return Err(invalid("bad extent header"));
        }
        let entries = u16le(node, 2) as usize;
        let depth = u16le(node, 6);
        for i in 0..entries {
            let e = &node[12 + i * 12..24 + i * 12];
            if depth == 0 {
                let mut len = u16le(e, 4) as u64;
                if len > 32768 {
                    len -= 32768; // uninitialized extent: reads as zeros
                    out.push((u32le(e, 0) as u64, 0, len));
                    continue;
                }
                let start = u32le(e, 8) as u64 | (u16le(e, 6) as u64) << 32;
                out.push((u32le(e, 0) as u64, start, len));
            } else {
                let leaf = u32le(e, 4) as u64 | (u16le(e, 8) as u64) << 32;
                let child = self.read_block(leaf)?;
                self.walk_extent_node(&child, out, level + 1)?;
            }
        }
        Ok(())
    }

    /// Whole contents of a regular file / directory / slow symlink.
    pub fn read_file(&mut self, ino: &Inode) -> io::Result<Vec<u8>> {
        let size = ino.size as usize;
        if ino.flags & INLINE_DATA_FL != 0 {
            let mut data = ino.block[..size.min(60)].to_vec();
            if size > 60 {
                if let Some(rest) = self.xattrs(ino)?.get("system.data") {
                    data.extend_from_slice(rest);
                }
            }
            data.truncate(size);
            return Ok(data);
        }
        let mut data = vec![0u8; size];
        let bs = self.block_size;
        for (logical, phys, count) in self.extents(ino)? {
            let start = (logical * bs) as usize;
            if start >= size || phys == 0 {
                continue;
            }
            let len = ((count * bs) as usize).min(size - start);
            self.read_at(phys * bs, &mut data[start..start + len])?;
        }
        Ok(data)
    }

    /// Streams a regular file into `out`, extent by extent (holes stay
    /// holes in the output if it is a seekable file).
    pub fn copy_file(&mut self, ino: &Inode, out: &mut File) -> io::Result<()> {
        use std::io::Write;
        if ino.flags & INLINE_DATA_FL != 0 {
            let d = self.read_file(ino)?;
            return out.write_all(&d);
        }
        let bs = self.block_size;
        let size = ino.size;
        let mut buf = vec![0u8; 1 << 20];
        for (logical, phys, count) in self.extents(ino)? {
            if phys == 0 {
                continue;
            }
            let start = logical * bs;
            if start >= size {
                continue;
            }
            let total = (count * bs).min(size - start);
            let mut done = 0u64;
            while done < total {
                let n = (total - done).min(buf.len() as u64) as usize;
                self.read_at(phys * bs + done, &mut buf[..n])?;
                out.seek(SeekFrom::Start(start + done))?;
                out.write_all(&buf[..n])?;
                done += n as u64;
            }
        }
        out.set_len(size)?;
        Ok(())
    }

    /// Directory entries (name, inode, file type), excluding "." and "..".
    pub fn read_dir(&mut self, ino: &Inode) -> io::Result<Vec<(String, u32)>> {
        let data = self.read_file(ino)?;
        let mut out = Vec::new();
        let bs = if ino.flags & INLINE_DATA_FL != 0 {
            data.len().max(1)
        } else {
            self.block_size as usize
        };
        let mut blk = 0;
        // Inline directories start with a 4-byte parent inode number.
        if ino.flags & INLINE_DATA_FL != 0 {
            let mut off = 4;
            while off + 8 <= data.len() {
                let inode = u32le(&data, off);
                let rec = u16le(&data, off + 4) as usize;
                let nlen = data[off + 6] as usize;
                if rec < 8 {
                    break;
                }
                if inode != 0 && off + 8 + nlen <= data.len() {
                    out.push((
                        String::from_utf8_lossy(&data[off + 8..off + 8 + nlen]).into_owned(),
                        inode,
                    ));
                }
                off += rec;
            }
            return Ok(out);
        }
        while blk * bs < data.len() {
            let block = &data[blk * bs..((blk + 1) * bs).min(data.len())];
            let mut off = 0;
            while off + 8 <= block.len() {
                let inode = u32le(block, off);
                let rec = u16le(block, off + 4) as usize;
                let nlen = block[off + 6] as usize;
                if rec < 8 || off + rec > block.len() {
                    break;
                }
                if inode != 0 && nlen > 0 && off + 8 + nlen <= block.len() {
                    let name =
                        String::from_utf8_lossy(&block[off + 8..off + 8 + nlen]).into_owned();
                    if name != "." && name != ".." {
                        out.push((name, inode));
                    }
                }
                off += rec;
            }
            blk += 1;
        }
        Ok(out)
    }

    pub fn read_link(&mut self, ino: &Inode) -> io::Result<String> {
        let data = if ino.size < 60 && ino.flags & (EXTENTS_FL | INLINE_DATA_FL) == 0 {
            ino.block[..ino.size as usize].to_vec()
        } else {
            self.read_file(ino)?
        };
        Ok(String::from_utf8_lossy(&data).into_owned())
    }

    /// Extended attributes ("security.selinux" etc.).
    pub fn xattrs(&mut self, ino: &Inode) -> io::Result<HashMap<String, Vec<u8>>> {
        let mut out = HashMap::new();
        // In-inode: after i_extra_isize.
        if ino.extra.len() >= 4 {
            let extra_isize = u16le(&ino.extra, 0) as usize;
            let start = extra_isize;
            if start + 4 <= ino.extra.len() && u32le(&ino.extra, start) == 0xEA02_0000 {
                let region = &ino.extra[start + 4..];
                parse_entries(region, region, &mut out);
            }
        }
        if ino.file_acl != 0 {
            let b = self.read_block(ino.file_acl)?;
            if u32le(&b, 0) == 0xEA02_0000 {
                parse_entries(&b[32..], &b, &mut out);
            }
        }
        Ok(out)
    }
}

fn prefix(index: u8) -> &'static str {
    match index {
        1 => "user.",
        2 => "system.posix_acl_access",
        3 => "system.posix_acl_default",
        4 => "trusted.",
        6 => "security.",
        7 => "system.",
        8 => "system.richacl",
        _ => "",
    }
}

/// Entries start at `entries`; value offsets are relative to `values`.
fn parse_entries(entries: &[u8], values: &[u8], out: &mut HashMap<String, Vec<u8>>) {
    let mut off = 0;
    while off + 16 <= entries.len() {
        let name_len = entries[off] as usize;
        if name_len == 0 && entries[off + 1] == 0 {
            break;
        }
        let index = entries[off + 1];
        let voff = u16le(entries, off + 2) as usize;
        let vsize = u32le(entries, off + 8) as usize;
        if off + 16 + name_len > entries.len() {
            break;
        }
        let name = format!(
            "{}{}",
            prefix(index),
            String::from_utf8_lossy(&entries[off + 16..off + 16 + name_len])
        );
        if voff + vsize <= values.len() {
            out.insert(name, values[voff..voff + vsize].to_vec());
        }
        off += (16 + name_len + 3) & !3;
    }
}
