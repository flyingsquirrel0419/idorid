//! `idorid` command-line runner (headless; used for tests and development).

use std::process::ExitCode;
use std::rc::Rc;

use idorid_vm::Vm;

fn usage() -> ExitCode {
    eprintln!(
        "usage:\n  idorid main <file.dex|file.apk>... <MainClass>   run static main(String[])\n  idorid run <app.apk> [script...]                 launch an app headless; commands from args or stdin:
                                                   window <w> <h> | size <id> <w> <h> | touch <id> <action> <x> <y> | click <id> | type <id> <text> | back | tick | json | log | quit
  idorid dump <file.dex>                           list classes and methods\n  idorid missing <app.apk|dex>...                  framework classes/methods/fields the app needs but core lacks\n\nflags: --stats  print instruction count, GC and timing"
    );
    ExitCode::from(2)
}

fn load(vm: &mut Vm, path: &str) -> Result<(), String> {
    let bytes = std::fs::read(path).map_err(|e| format!("{path}: {e}"))?;
    if bytes.starts_with(b"PK") {
        let apk = idorid_apk::Apk::open(bytes).map_err(|e| format!("{path}: {e:?}"))?;
        for d in apk.dex_files() {
            vm.add_dex(Rc::from(d))?;
        }
        Ok(())
    } else {
        vm.add_dex(Rc::from(bytes))
    }
}

fn main() -> ExitCode {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    let stats = args.iter().any(|a| a == "--stats");
    args.retain(|a| a != "--stats");
    let Some(cmd) = args.first().cloned() else {
        return usage();
    };
    match cmd.as_str() {
        "missing" if args.len() >= 2 => {
            // Framework APIs referenced by the app but absent from core.dex.
            let mut vm = Vm::new();
            let first = vm.dexes.len() as u16;
            for p in &args[1..] {
                if let Err(e) = load(&mut vm, p) {
                    eprintln!("error: {e}");
                    return ExitCode::from(1);
                }
            }
            let mut all = Vec::new();
            for d in first..vm.dexes.len() as u16 {
                all.extend(vm.missing_report(d));
            }
            all.sort();
            all.dedup();
            for l in &all {
                println!("{l}");
            }
            eprintln!("{} missing", all.len());
            ExitCode::SUCCESS
        }
        "main" if args.len() >= 3 => {
            let t0 = std::time::Instant::now();
            let mut vm = Vm::new();
            for p in &args[1..args.len() - 1] {
                if let Err(e) = load(&mut vm, p) {
                    eprintln!("error: {e}");
                    return ExitCode::from(1);
                }
            }
            let t_load = t0.elapsed();
            let r = vm.run_main(&args[args.len() - 1]);
            if stats {
                eprintln!(
                    "[idorid] load {:?}, total {:?}, {} instructions, {} classes, {} objects live, {} GCs, heap {} KB",
                    t_load,
                    t0.elapsed(),
                    vm.instructions,
                    vm.classes.len(),
                    vm.heap.object_count(),
                    vm.heap.collections,
                    vm.heap.live_bytes / 1024
                );
            }
            match r {
                Ok(()) => ExitCode::from(vm.exit_code.unwrap_or(0) as u8),
                Err(e) => {
                    eprintln!("Exception in thread \"main\" {e}");
                    ExitCode::from(1)
                }
            }
        }
        "run" if args.len() >= 2 => run_app(&args[1], &args[2..], stats),
        "dump" if args.len() == 2 => {
            let bytes = match std::fs::read(&args[1]) {
                Ok(b) => b,
                Err(e) => {
                    eprintln!("{e}");
                    return ExitCode::from(1);
                }
            };
            let dex = match idorid_dex::Dex::parse(&bytes) {
                Ok(d) => d,
                Err(e) => {
                    eprintln!("{e}");
                    return ExitCode::from(1);
                }
            };
            println!("{dex:?}");
            for def in dex.class_defs().flatten() {
                println!("{}", dex.type_descriptor(def.class_idx).unwrap_or_default());
                if let Ok(cd) = dex.class_data(def.class_data_off) {
                    for m in cd.direct_methods.iter().chain(&cd.virtual_methods) {
                        if let Ok(mid) = dex.method_id(m.method_idx) {
                            println!(
                                "  {}{}",
                                dex.string(mid.name_idx).unwrap_or_default(),
                                dex.proto_signature(mid.proto_idx as u32)
                                    .unwrap_or_default()
                            );
                        }
                    }
                }
            }
            ExitCode::SUCCESS
        }
        _ => usage(),
    }
}

fn render(v: &serde_json::Value, depth: usize, out: &mut String) {
    let ty = v["type"].as_str().unwrap_or("?");
    let id = v["id"].as_i64().unwrap_or(0);
    let mut line = format!("{}#{id} {ty}", "  ".repeat(depth));
    if let Some(ops) = v.get("draw").and_then(|d| d.as_array()) {
        line.push_str(&format!(" [canvas {} ops]", ops.len()));
    }
    if let Some(t) = v.get("text").and_then(|t| t.as_str()) {
        line.push_str(&format!(" {t:?}"));
    }
    if let Some(h) = v.get("hint").and_then(|t| t.as_str()) {
        line.push_str(&format!(" hint={h:?}"));
    }
    if let Some(c) = v.get("checked") {
        line.push_str(&format!(" checked={c}"));
    }
    if v.get("visible").is_some() {
        line.push_str(&format!(" visible={}", v["visible"]));
    }
    if v.get("clickable").is_some()
        && !matches!(ty, "Button" | "CheckBox" | "Switch" | "RadioButton")
    {
        line.push_str(" [clickable]");
    }
    out.push_str(&line);
    out.push('\n');
    if let Some(ch) = v.get("children").and_then(|c| c.as_array()) {
        for c in ch {
            render(c, depth + 1, out);
        }
    }
}

fn show(json: &str, raw: bool) {
    if raw {
        println!("{json}");
        return;
    }
    let v: serde_json::Value = match serde_json::from_str(json) {
        Ok(v) => v,
        Err(e) => {
            println!("(bad json: {e}) {json}");
            return;
        }
    };
    let mut out = format!("== {} ==", v["title"].as_str().unwrap_or(""));
    if v["finished"].as_bool() == Some(true) {
        out.push_str(" [finished]");
    }
    out.push('\n');
    if let Some(e) = v["error"].as_str() {
        out.push_str(&format!("!! error: {e}\n"));
    }
    if !v["root"].is_null() {
        render(&v["root"], 0, &mut out);
    }
    if let Some(t) = v.get("toasts").and_then(|t| t.as_array()) {
        for x in t {
            out.push_str(&format!("[toast] {}\n", x.as_str().unwrap_or("")));
        }
    }
    if let Some(d) = v.get("dialog") {
        out.push_str(&format!(
            "[dialog] {} — {}\n",
            d["title"].as_str().unwrap_or(""),
            d["message"].as_str().unwrap_or("")
        ));
        if let Some(b) = d["buttons"].as_array() {
            for x in b {
                out.push_str(&format!(
                    "  button {} {:?}\n",
                    x["which"],
                    x["text"].as_str().unwrap_or("")
                ));
            }
        }
    }
    print!("{out}");
}

fn run_app(path: &str, script: &[String], stats: bool) -> ExitCode {
    let bytes = match std::fs::read(path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("{path}: {e}");
            return ExitCode::from(1);
        }
    };
    let t0 = std::time::Instant::now();
    let mut app = match idorid_ffi::App::load(bytes, true) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("error: {e}");
            return ExitCode::from(1);
        }
    };
    let raw = std::env::var("IDORID_JSON").is_ok();
    if let Ok(dir) = std::env::var("IDORID_DATA") {
        app.set_data_dir(&dir);
    }
    let ui = app.launch();
    let launch_time = t0.elapsed();
    show(&ui, raw);
    if stats {
        eprintln!(
            "[idorid] launch {:?}, {} instructions, {} classes, {} objects, heap {} KB",
            launch_time,
            app.vm.instructions,
            app.vm.classes.len(),
            app.vm.heap.object_count(),
            app.vm.heap.live_bytes / 1024
        );
    }
    let lines: Box<dyn Iterator<Item = String>> = if script.is_empty() {
        Box::new(std::io::stdin().lines().map_while(Result::ok))
    } else {
        Box::new(script.iter().flat_map(|s| {
            s.split(';')
                .map(|x| x.trim().to_string())
                .collect::<Vec<_>>()
        }))
    };
    for line in lines {
        let mut parts = line.splitn(3, ' ');
        let cmd = parts.next().unwrap_or("");
        let a1 = parts.next().unwrap_or("");
        let rest = parts.next().unwrap_or("");
        let out = match cmd {
            "window" => {
                app.set_window_size(a1.parse().unwrap_or(0), rest.trim().parse().unwrap_or(0))
            }
            "size" => {
                let mut p = rest.split_whitespace();
                app.set_size(
                    a1.parse().unwrap_or(0),
                    p.next().unwrap_or("").parse().unwrap_or(0),
                    p.next().unwrap_or("").parse().unwrap_or(0),
                )
            }
            "touch" => {
                let mut p = rest.split_whitespace();
                app.touch(
                    a1.parse().unwrap_or(0),
                    p.next().unwrap_or("").parse().unwrap_or(0),
                    p.next().unwrap_or("").parse().unwrap_or(0.0),
                    p.next().unwrap_or("").parse().unwrap_or(0.0),
                )
            }
            "tap" => app.tap(
                a1.parse().unwrap_or(0.0),
                rest.split_whitespace()
                    .next()
                    .unwrap_or("")
                    .parse()
                    .unwrap_or(0.0),
            ),
            "click" => app.click(a1.parse().unwrap_or(0)),
            "long" => app.long_click(a1.parse().unwrap_or(0)),
            "type" => app.set_text(a1.parse().unwrap_or(0), rest),
            "enter" => app.editor_action(a1.parse().unwrap_or(0)),
            "back" => app.back(),
            "tick" => app.tick(),
            "json" => {
                println!("{}", app.ui());
                continue;
            }
            "log" => {
                print!("{}", app.take_log());
                continue;
            }
            "quit" | "exit" => break,
            "" => continue,
            _ => {
                eprintln!("unknown command: {cmd}");
                continue;
            }
        };
        println!("> {line}");
        show(&out, raw);
    }
    if stats {
        eprintln!(
            "[idorid] total {:?}, {} instructions, {} GCs",
            t0.elapsed(),
            app.vm.instructions,
            app.vm.heap.collections
        );
    }
    ExitCode::SUCCESS
}
