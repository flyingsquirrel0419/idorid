//! Software MMU interface.

pub const PAGE_SHIFT: u32 = 12;
pub const PAGE_SIZE: u64 = 1 << PAGE_SHIFT;
const PAGE_MASK: u64 = PAGE_SIZE - 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Access {
    Read,
    Write,
    Exec,
}

/// A guest address space, implemented by the embedder.
///
/// `translate` returns a host pointer to the start of the 4 KiB host buffer
/// backing guest page `page` (guest address >> 12) if the page is mapped with
/// the requested permission. The pointer must stay valid until
/// `generation()` changes (munmap/mprotect/mremap bump it), so that TLBs can
/// cache it.
///
/// Implementations are shared between threads of a guest process; the CPU
/// only ever writes through returned pointers (guest memory is plain bytes,
/// races between guest threads are the guest's business, and atomics use host
/// atomic instructions on these pointers).
pub trait AddressSpace {
    fn translate(&self, page: u64, access: Access) -> Option<*mut u8>;
    /// Monotonic counter, bumped whenever an existing translation may become
    /// invalid.
    fn generation(&self) -> u64;
}

const TLB_SIZE: usize = 256;

#[derive(Clone, Copy)]
struct Entry {
    tag: u64, // page number + 1 (0 = empty)
    host: *mut u8,
}

const EMPTY: Entry = Entry {
    tag: 0,
    host: core::ptr::null_mut(),
};

/// Direct-mapped translation cache (separate read / write / exec arrays).
pub struct Tlb {
    read: Box<[Entry; TLB_SIZE]>,
    write: Box<[Entry; TLB_SIZE]>,
    exec: Box<[Entry; TLB_SIZE]>,
    generation: u64,
}

// The host pointers are owned by the AddressSpace; a Tlb is per guest thread.
unsafe impl Send for Tlb {}

/// Cloning gives an empty TLB (translations are per thread).
impl Clone for Tlb {
    fn clone(&self) -> Tlb {
        Tlb::new()
    }
}

impl Default for Tlb {
    fn default() -> Self {
        Self::new()
    }
}

impl Tlb {
    pub fn new() -> Tlb {
        Tlb {
            read: Box::new([EMPTY; TLB_SIZE]),
            write: Box::new([EMPTY; TLB_SIZE]),
            exec: Box::new([EMPTY; TLB_SIZE]),
            generation: u64::MAX,
        }
    }

    pub fn flush(&mut self) {
        self.read.fill(EMPTY);
        self.write.fill(EMPTY);
        self.exec.fill(EMPTY);
    }

    /// Flushes if the address space changed since the last call.
    #[inline]
    pub fn sync<A: AddressSpace + ?Sized>(&mut self, mem: &A) {
        let g = mem.generation();
        if g != self.generation {
            self.flush();
            self.generation = g;
        }
    }

    /// Host pointer for guest address `addr`, valid for the rest of its page.
    #[inline(always)]
    pub fn lookup<A: AddressSpace + ?Sized>(
        &mut self,
        mem: &A,
        addr: u64,
        access: Access,
    ) -> Option<*mut u8> {
        let page = addr >> PAGE_SHIFT;
        let idx = (page as usize) & (TLB_SIZE - 1);
        let arr = match access {
            Access::Read => &mut self.read,
            Access::Write => &mut self.write,
            Access::Exec => &mut self.exec,
        };
        let e = arr[idx];
        if e.tag == page + 1 {
            return Some(unsafe { e.host.add((addr & PAGE_MASK) as usize) });
        }
        let host = mem.translate(page, access)?;
        // The translation itself may have changed the address space (e.g. a
        // copy-on-write copy): drop stale entries before caching.
        if mem.generation() != self.generation {
            self.flush();
            self.generation = mem.generation();
        }
        let arr = match access {
            Access::Read => &mut self.read,
            Access::Write => &mut self.write,
            Access::Exec => &mut self.exec,
        };
        arr[idx] = Entry {
            tag: page + 1,
            host,
        };
        Some(unsafe { host.add((addr & PAGE_MASK) as usize) })
    }
}
