//! Run static ELFs through generated units and compare with QEMU.
#[path = "../../a64/tests/support/mod.rs"]
mod support;
use idorid_a64::{AddressSpace, Cpu, Exit};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Mutex,
};
static UNIT: Mutex<Option<idorid_a64::mem::AotUnit>> = Mutex::new(None);
static CALLS: AtomicUsize = AtomicUsize::new(0);
fn counted(c: &mut Cpu, m: &dyn AddressSpace, b: u64) -> Option<Exit> {
    CALLS.fetch_add(1, Ordering::Relaxed);
    (UNIT.lock().unwrap().unwrap().execute)(c, m, b)
}
#[test]
fn generated_static_elf_matches_qemu() {
    let Some(root) = std::env::var_os("IDORID_AOT_ROM") else {
        eprintln!("no generated test profile");
        return;
    };
    let mut tested = 0;
    for name in ["hello", "simd_crypto"] {
        let guest_path = format!("/bin/{name}");
        let Some(unit) = idorid_aot::lookup(&guest_path, 0) else {
            continue;
        };
        *UNIT.lock().unwrap() = Some(unit);
        let before = CALLS.load(Ordering::Relaxed);
        let path = std::path::Path::new(&root).join("bin").join(name);
        let elf = std::fs::read(&path).unwrap();
        let qemu = std::process::Command::new("qemu-aarch64-static")
            .arg(&path)
            .output()
            .unwrap();
        let o = support::run_elf_with_aot(&elf, &[name], 20_000_000_000, |offset| {
            if offset == 0 {
                Some(idorid_a64::mem::AotUnit {
                    execute: counted,
                    ..unit
                })
            } else {
                None
            }
        });
        assert_eq!(o.stdout, qemu.stdout);
        assert_eq!(Some(o.exit_code), qemu.status.code());
        assert!(
            CALLS.load(Ordering::Relaxed) > before,
            "generated {name} unit was not entered"
        );
        tested += 1;
    }
    assert!(tested > 0, "test profile did not select a CPU test ELF");
}
