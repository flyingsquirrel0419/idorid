# ROM page AOT

Collect a profile with `IDORID_PROFILE=/tmp/idorid.prof` while running the
interpreter. Build selected pages into the CLI with:

```sh
IDORID_AOT_PROFILE=/tmp/idorid.prof IDORID_AOT_ROM="$HOME/aosp/rom13" \
IDORID_AOT_LIMIT=4 cargo build -j2 --profile fast -p idorid-cli
```

The build script emits Rust state machines for the hottest file-backed pages.
The default limit is one; increase it deliberately because each additional page
increases compile time and binary size. With no profile and ROM variables, the
table is empty and execution uses the interpreter. `IDORID_DISABLE_AOT=1`
selects the interpreter at runtime for comparisons; `IDORID_AOT_TRACE=1` logs
the first entered unit. `IDORID_AOT_ONLY_OFFSET=0x11a000` restricts dispatch
to one file-page offset when measuring individual units. The executable mapping must be non-writable, and the
runtime checks its contents before dispatch; mapping-generation changes flush
cached translations.

For inspection or integration into another build, generate source directly:

```sh
cargo run -j2 -p idorid-aot --bin generate -- /tmp/idorid.prof "$HOME/aosp/rom13" /tmp/units.rs 4
```
