//! AArch64 (A64, EL0) user-mode CPU interpreter.
//!
//! No code is ever generated at run time: guest instructions are decoded and
//! executed by Rust code. (A later build-time AOT translator may emit host code
//! for a fixed ROM; see docs/NEO.md.)
//!
//! The CPU knows nothing about Linux. It runs until something needs the
//! "kernel" (an `svc`, a fault, an undefined instruction, ...) and returns an
//! [`Exit`]. The embedder (crates/kernel) handles it and calls [`Cpu::run`]
//! again.
//!
//! Memory is reached through a software MMU: the embedder implements
//! [`AddressSpace`], which maps 4 KiB guest pages to host memory, and the CPU
//! caches translations in a per-CPU [`Tlb`].

mod access;
mod base_atomic;
mod base_cache;
mod base_crypto;
pub mod cpu;
mod exec;
mod fp;
mod fp_estimate;
mod fp_half;
mod fp_vector;
mod fp_vector_half;
pub mod mem;
mod simd;
mod simd_crypto;

pub use cpu::{Cpu, Exit};
pub use mem::{Access, AddressSpace, Tlb, PAGE_SHIFT, PAGE_SIZE};

/// Entry points used by generated ahead-of-time page units.
pub mod aot {
    pub use crate::exec::{aot_simd_simple, aot_simd_unretired, aot_step};
}
