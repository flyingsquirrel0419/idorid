mod support;

#[test]
fn hello() {
    support::differential("hello", &["-O2"]);
}

#[test]
fn base_integer_o0() {
    support::differential("base_integer", &["-O0", "-nostdlib", "-mgeneral-regs-only"]);
}
#[test]
fn base_integer_o2() {
    support::differential("base_integer", &["-O2", "-nostdlib", "-mgeneral-regs-only"]);
}
#[test]
fn base_integer_o3() {
    support::differential("base_integer", &["-O3", "-nostdlib", "-mgeneral-regs-only"]);
}

#[test]
fn base_flags_system_and_atomics() {
    use idorid_a64::{Cpu, Exit};
    let mem = support::Pages::new();
    mem.map(0x1000, 0x2000, 7);
    let instructions = [
        0xab010002u32,
        0xd51bd041,
        0xd53bd042,
        0xc85f7c02,
        0xc8037c01,
        0xf8210002,
        0xd50b7420,
    ];
    for (i, word) in instructions.iter().enumerate() {
        mem.write(0x1000 + 4 * i as u64, &word.to_le_bytes());
    }
    let mut cpu = Cpu::new();
    cpu.pc = 0x1000;
    cpu.x[0] = u64::MAX;
    cpu.x[1] = 1;
    assert_eq!(cpu.run(&mem, 1), Exit::Budget);
    assert_eq!(cpu.x[2], 0);
    assert_eq!(cpu.nzcv, 0x60000000);
    cpu.x[1] = 123;
    assert_eq!(cpu.run(&mem, 2), Exit::Budget);
    assert_eq!(cpu.x[2], 123);
    cpu.x[0] = 0x2000;
    cpu.x[1] = 99;
    mem.write(0x2000, &42u64.to_le_bytes());
    assert_eq!(cpu.run(&mem, 2), Exit::Budget);
    assert_eq!(cpu.x[2], 42);
    assert_eq!(cpu.x[3], 0);
    assert_eq!(mem.read(0x2000, 8).unwrap(), 99u64.to_le_bytes());
    cpu.x[1] = 3;
    assert_eq!(cpu.run(&mem, 1), Exit::Budget);
    assert_eq!(cpu.x[2], 99);
    assert_eq!(mem.read(0x2000, 8).unwrap(), 102u64.to_le_bytes());
    assert_eq!(cpu.run(&mem, 1), Exit::Budget);
    assert_eq!(mem.read(0x2000, 64).unwrap(), vec![0; 64]);
}

#[test]
fn base_extensions() {
    support::differential(
        "base_extensions",
        &[
            "-O2",
            "-nostdlib",
            "-mgeneral-regs-only",
            "-march=armv8.5-a",
        ],
    );
}

#[test]
fn base_libc_o0() {
    support::differential(
        "base_libc",
        &["-O0", "-mgeneral-regs-only", "-march=armv8-a"],
    );
}
#[test]
fn base_libc_o2() {
    support::differential(
        "base_libc",
        &["-O2", "-mgeneral-regs-only", "-march=armv8-a"],
    );
}
#[test]
fn base_libc_lse() {
    support::differential(
        "base_libc",
        &["-O3", "-mgeneral-regs-only", "-march=armv8.1-a"],
    );
}

#[test]
fn base_crc() {
    support::differential(
        "base_crc",
        &[
            "-O2",
            "-nostdlib",
            "-mgeneral-regs-only",
            "-march=armv8-a+crc",
        ],
    );
}

#[test]
fn base_lse() {
    support::differential(
        "base_lse",
        &[
            "-O2",
            "-nostdlib",
            "-mgeneral-regs-only",
            "-march=armv8.1-a",
        ],
    );
}

#[test]
fn base_memory() {
    support::differential(
        "base_memory",
        &[
            "-O2",
            "-nostdlib",
            "-mgeneral-regs-only",
            "-march=armv8.5-a",
        ],
    );
}

#[test]
fn base_sieve_o0() {
    support::differential("base_sieve", &["-O0", "-mgeneral-regs-only"]);
}
#[test]
fn base_sieve_o2() {
    support::differential("base_sieve", &["-O2", "-mgeneral-regs-only"]);
}
#[test]
fn base_sieve_o3() {
    support::differential("base_sieve", &["-O3", "-mgeneral-regs-only"]);
}

#[test]
fn exclusive_failure_clears_monitor() {
    use idorid_a64::{Cpu, Exit};
    let mem = support::Pages::new();
    mem.map(0x1000, 0x2000, 7);
    for (n, i) in [0xc85f7c02u32, 0xc8037c01, 0xc8037c01].iter().enumerate() {
        mem.write(0x1000 + n as u64 * 4, &i.to_le_bytes());
    }
    mem.write(0x2000, &1u64.to_le_bytes());
    let mut cpu = Cpu::new();
    cpu.pc = 0x1000;
    cpu.x[0] = 0x2000;
    cpu.x[1] = 3;
    assert_eq!(cpu.run(&mem, 1), Exit::Budget);
    mem.write(0x2000, &2u64.to_le_bytes());
    assert_eq!(cpu.run(&mem, 1), Exit::Budget);
    assert_eq!(cpu.x[3], 1);
    mem.write(0x2000, &1u64.to_le_bytes());
    assert_eq!(cpu.run(&mem, 1), Exit::Budget);
    assert_eq!(cpu.x[3], 1);
}

#[test]
fn tagged_data_and_untagged_instruction_fetch() {
    use idorid_a64::{Access, Cpu, Exit};
    let mem = support::Pages::new();
    mem.map(0x1000, 0x3000, 7);
    // ldr x2,[x0]; str x2,[x1]; ldxr x2,[x0]; stxr w3,x2,[x1]
    for (n, insn) in [0xf9400002u32, 0xf9000022, 0xc85f7c02, 0xc8037c22]
        .iter()
        .enumerate()
    {
        mem.write(0x1000 + n as u64 * 4, &insn.to_le_bytes());
    }
    mem.write(0x2000, &0x123456789abcdef0u64.to_le_bytes());
    let mut c = Cpu::new();
    c.pc = 0x1000;
    c.x[0] = 0xb400000000002000;
    c.x[1] = 0xaa00000000002000;
    assert_eq!(c.run(&mem, 4), Exit::Budget);
    assert_eq!(c.x[2], 0x123456789abcdef0);
    assert_eq!(c.x[3], 0);
    c.pc = 0xb400000000001000;
    assert_eq!(
        c.run(&mem, 1),
        Exit::Fault {
            addr: c.pc,
            access: Access::Exec
        }
    );
    c.pc = 0x1000;
    c.x[0] = 0xb400000000005000;
    assert_eq!(
        c.run(&mem, 1),
        Exit::Fault {
            addr: 0x5000,
            access: Access::Read
        }
    );
}

#[test]
fn tagged_simd_load() {
    use idorid_a64::{Cpu, Exit};
    let mem = support::Pages::new();
    mem.map(0x1000, 0x2000, 7);
    mem.write(0x1000, &0x3dc00000u32.to_le_bytes()); // ldr q0,[x0]
    let value = 0x123456789abcdef00123456789abcdefu128;
    mem.write(0x2000, &value.to_le_bytes());
    let mut c = Cpu::new();
    c.pc = 0x1000;
    c.x[0] = 0xb400000000002000;
    assert_eq!(c.run(&mem, 1), Exit::Budget);
    assert_eq!(c.v[0], value);
}

#[test]
fn base_sha256_o0() {
    support::differential("base_sha256", &["-O0", "-mgeneral-regs-only"]);
}
#[test]
fn base_sha256_o2() {
    support::differential("base_sha256", &["-O2", "-mgeneral-regs-only"]);
}
#[test]
fn base_sha256_o3() {
    support::differential("base_sha256", &["-O3", "-mgeneral-regs-only"]);
}

/// Run with `cargo test --profile fast -p idorid-a64 --test differential benchmark_mips -- --ignored --nocapture`.
#[test]
#[ignore = "performance benchmark"]
fn benchmark_mips() {
    use std::process::Command;
    use std::time::Instant;
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let out = std::path::Path::new(env!("CARGO_TARGET_TMPDIR")).join("a64-bench");
    std::fs::create_dir_all(&out).unwrap();
    for name in [
        "base_sieve",
        "base_sha256",
        "base_libc",
        "base_benchsort",
        "base_benchmem",
        "base_bytecode",
    ] {
        let bin = out.join(name);
        let status = Command::new("aarch64-linux-gnu-gcc")
            .args(["-O2", "-static", "-mgeneral-regs-only"])
            .arg(root.join("tests/c").join(format!("{name}.c")))
            .arg("-o")
            .arg(&bin)
            .status()
            .expect("aarch64 cross compiler");
        assert!(status.success());
        let elf = std::fs::read(&bin).unwrap();
        let started = Instant::now();
        let result = support::run_elf(&elf, &[name], 20_000_000_000);
        let seconds = started.elapsed().as_secs_f64();
        assert_eq!(result.exit_code, 0);
        eprintln!(
            "{name}: {:.2} MIPS ({} instructions, {:.3}s)",
            result.instructions as f64 / seconds / 1e6,
            result.instructions,
            seconds
        );
    }
}

#[test]
fn cached_blocks_budget_fault_and_generation() {
    use idorid_a64::{Access, Cpu, Exit};
    let mem = support::Pages::new();
    mem.map(0x1000, 0x1000, 7);
    // add x0,x0,#1; add x0,x0,#1; ldr x1,[x2]
    for (n, i) in [0x91000400u32, 0x91000400, 0xf9400041].iter().enumerate() {
        mem.write(0x1000 + n as u64 * 4, &i.to_le_bytes());
    }
    let mut c = Cpu::new();
    c.pc = 0x1000;
    c.x[2] = 0x5000;
    assert_eq!(c.run(&mem, 1), Exit::Budget);
    assert_eq!(c.pc, 0x1004);
    assert_eq!(c.retired, 1);
    assert_eq!(c.x[0], 1);
    assert_eq!(
        c.run(&mem, 9),
        Exit::Fault {
            addr: 0x5000,
            access: Access::Read
        }
    );
    assert_eq!(c.pc, 0x1008);
    assert_eq!(c.retired, 2);
    assert_eq!(c.x[0], 2);
    // Generation change invalidates a previously cached translation.
    mem.write(0x1000, &0x91001400u32.to_le_bytes());
    mem.map(0x1000, 0x1000, 7);
    c.pc = 0x1000;
    assert_eq!(c.run(&mem, 1), Exit::Budget);
    assert_eq!(c.x[0], 7);
    // A generation bump also invalidates cached execute permission.
    mem.map(0x1000, 0x1000, 3);
    c.pc = 0x1000;
    assert_eq!(
        c.run(&mem, 1),
        Exit::Fault {
            addr: 0x1000,
            access: Access::Exec
        }
    );
    assert_eq!(c.retired, 3);
}
