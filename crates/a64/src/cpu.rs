//! Architectural state and the run loop entry point.

use crate::mem::{Access, AddressSpace, Tlb};

/// Why [`Cpu::run`] returned. `pc` always points at the instruction that
/// caused the exit, except for `Svc`, where it already points after the `svc`
/// (like ELR_EL1 on real hardware).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Exit {
    /// `svc #imm`. Linux syscall number is in x8, args in x0..x5.
    Svc(u16),
    /// Translation or permission fault (a SIGSEGV for the guest).
    Fault { addr: u64, access: Access },
    /// Misaligned access where the architecture requires alignment
    /// (exclusives, atomics, SP misalignment...): SIGBUS.
    Alignment { addr: u64 },
    /// `brk #imm` (SIGTRAP).
    Brk(u16),
    /// `udf` or an encoding we do not implement (SIGILL). The raw word is
    /// included for diagnostics.
    Undefined(u32),
    /// `wfe`/`wfi`/`yield`: a scheduling hint, the embedder may switch threads.
    Yield,
    /// The instruction budget passed to `run` is used up.
    Budget,
}

/// One guest hardware thread (EL0 view).
#[derive(Clone)]
pub struct Cpu {
    /// x0..x30. x31 is SP or XZR depending on the encoding.
    pub x: [u64; 31],
    pub sp: u64,
    pub pc: u64,
    /// NZCV in bits 31..28 (same layout as the NZCV system register).
    pub nzcv: u64,
    /// SIMD/FP registers v0..v31 (128-bit; lane 0 = low bits).
    pub v: [u128; 32],
    pub fpcr: u64,
    pub fpsr: u64,
    pub tpidr_el0: u64,
    pub tpidrro_el0: u64,
    /// Local exclusive monitor: (address, size) of the last LDXR/LDAXR.
    pub exclusive: Option<(u64, u8)>,
    /// Value observed by the local exclusive monitor.
    pub exclusive_value: u128,
    /// Guest instructions retired (statistics; also the basis of `Budget`).
    pub retired: u64,
    pub tlb: Tlb,
    pub(crate) block_cache: crate::base_cache::BlockCache,
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            x: [0; 31],
            sp: 0,
            pc: 0,
            nzcv: 0,
            v: [0; 32],
            fpcr: 0,
            fpsr: 0,
            tpidr_el0: 0,
            tpidrro_el0: 0,
            exclusive: None,
            exclusive_value: 0,
            retired: 0,
            tlb: Tlb::new(),
            block_cache: crate::base_cache::BlockCache::default(),
        }
    }

    /// Executes up to `budget` instructions.
    pub fn run<A: AddressSpace + ?Sized>(&mut self, mem: &A, budget: u64) -> Exit {
        self.tlb.sync(mem);
        crate::exec::run(self, mem, budget)
    }
}
