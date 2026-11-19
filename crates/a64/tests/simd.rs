mod support;
use idorid_a64::{Access, AddressSpace, Cpu, Exit};
use std::cell::UnsafeCell;

struct Memory(UnsafeCell<[u8; 4096]>);
impl AddressSpace for Memory {
    fn translate(&self, page: u64, _: Access) -> Option<*mut u8> {
        (page == 0).then(|| self.0.get().cast::<u8>())
    }
    fn generation(&self) -> u64 {
        0
    }
}
#[test]
fn instructions_through_dispatch() {
    let mem = Memory(UnsafeCell::new([0; 4096]));
    let words = [0x4e228420u32, 0x1e222823, 0xd4200000]; // add v0.16b; fadd s3; brk
    for (i, word) in words.iter().enumerate() {
        unsafe {
            (&mut *mem.0.get())[i * 4..i * 4 + 4].copy_from_slice(&word.to_le_bytes());
        }
    }
    let mut cpu = Cpu::new();
    cpu.v[1] = 0x3f800000;
    cpu.v[2] = 0x40000000;
    assert_eq!(cpu.run(&mem, 10), Exit::Brk(0));
    assert_eq!(cpu.v[3], 0x40400000);
    assert_eq!(cpu.v[0], 0x7f800000);
}

#[test]
fn differential_neon() {
    support::differential("simd_basic", &["-O2"]);
}

#[test]
fn differential_widening() {
    support::differential("simd_widen", &["-O2"]);
}

#[test]
fn differential_fp_conversions() {
    support::differential("fp_conversions", &["-O2"]);
}

#[test]
fn differential_fp_round() {
    support::differential("fp_round", &["-O2"]);
}

#[test]
fn differential_crypto() {
    support::differential("simd_crypto", &["-O2", "-march=armv8-a+crypto"]);
}

#[test]
fn differential_shifts() {
    support::differential("simd_shift", &["-O2"]);
}

#[test]
fn differential_saturation() {
    support::differential("simd_saturate", &["-O2"]);
}

#[test]
fn differential_fma() {
    support::differential("fp_fma", &["-O2"]);
}

#[test]
fn differential_max_min() {
    support::differential("fp_maxmin", &["-O2"]);
}

#[test]
fn differential_precision() {
    support::differential("fp_precision", &["-O2"]);
}

#[test]
fn differential_half_conversion() {
    support::differential("fp_half", &["-O2", "-march=armv8.2-a+fp16"]);
}

#[test]
fn differential_half_rounding() {
    support::differential("fp_half_round", &["-O2", "-march=armv8.2-a+fp16"]);
}

#[test]
fn differential_sha256() {
    support::differential("simd_sha", &["-O2", "-march=armv8-a+crypto"]);
}

#[test]
fn differential_vector_float() {
    support::differential("fp_vector", &["-O2"]);
}

#[test]
fn tagged_vector_load_uses_shared_access_path() {
    let mem = Memory(UnsafeCell::new([0; 4096]));
    let insn = 0x3dc00020u32; // ldr q0, [x1]
    unsafe {
        (&mut *mem.0.get())[0..4].copy_from_slice(&insn.to_le_bytes());
        (&mut *mem.0.get())[4..8].copy_from_slice(&0xd4200000u32.to_le_bytes());
        (&mut *mem.0.get())[0x100..0x110].copy_from_slice(&[0x5a; 16]);
    }
    let mut cpu = Cpu::new();
    cpu.x[1] = 0xb400000000000100;
    assert_eq!(cpu.run(&mem, 10), Exit::Brk(0));
    assert_eq!(cpu.v[0], u128::from_le_bytes([0x5a; 16]));
}

#[test]
fn differential_indexed_multiply() {
    support::differential("simd_index", &["-O2"]);
}

#[test]
fn differential_dot_product() {
    support::differential("simd_dot", &["-O2", "-march=armv8.2-a+dotprod"]);
}

#[test]
fn differential_sha3() {
    support::differential("simd_sha3", &["-O2", "-march=armv8.2-a+sha3"]);
}

#[test]
fn differential_variable_shifts() {
    support::differential("simd_varshift", &["-O2"]);
}

#[test]
fn differential_absolute_difference() {
    support::differential("simd_absdiff", &["-O2"]);
}

#[test]
fn differential_int_to_float_rounding() {
    support::differential("fp_int_round", &["-O2"]);
}

#[test]
fn differential_conditional_float() {
    support::differential("fp_conditional", &["-O2"]);
}

#[test]
fn differential_saturating_immediate_shifts() {
    support::differential("simd_satshift", &["-O2"]);
}

#[test]
fn differential_saturating_narrow_shifts() {
    support::differential("simd_satnarrow", &["-O2"]);
}

#[test]
fn differential_integer_estimates() {
    support::differential("simd_estimate", &["-O2"]);
}

#[test]
fn differential_float_estimates() {
    support::differential("fp_estimate", &["-O2"]);
}

#[test]
fn differential_double_estimates() {
    support::differential("fp_estimate_double", &["-O2"]);
}

#[test]
fn differential_estimate_steps() {
    support::differential("fp_steps", &["-O2"]);
}

#[test]
fn differential_half_arithmetic() {
    support::differential("fp_half_arith", &["-O2", "-march=armv8.2-a+fp16"]);
}

#[test]
fn differential_half_fma() {
    support::differential("fp_half_fma", &["-O2", "-march=armv8.2-a+fp16"]);
}

#[test]
fn differential_vector_conversions() {
    support::differential("fp_vector_convert", &["-O2", "-march=armv8.2-a+fp16"]);
}

#[test]
fn differential_arithmetic_flags() {
    support::differential("fp_flags", &["-O2"]);
}

#[test]
fn differential_halving_arithmetic() {
    support::differential("simd_halving", &["-O2"]);
}

#[test]
fn differential_vector_integer_conversions() {
    support::differential("fp_vec_int", &["-O2"]);
}

#[test]
fn differential_doubling_multiply() {
    support::differential("simd_dmul", &["-O2", "-march=armv8.1-a"]);
}

#[test]
fn differential_indexed_float() {
    support::differential("fp_index", &["-O2"]);
}

#[test]
fn differential_half_vector_arithmetic() {
    support::differential("fp_half_vector", &["-O2", "-march=armv8.2-a+fp16"]);
}

#[test]
fn differential_half_controls() {
    support::differential("fp_half_controls", &["-O2", "-march=armv8.2-a+fp16"]);
}

#[test]
fn differential_structures() {
    support::differential("simd_structures", &["-O2"]);
}

#[test]
fn differential_narrowing_add_subtract() {
    support::differential("simd_addhn", &["-O2"]);
}
