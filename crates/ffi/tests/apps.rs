//! End-to-end: real APKs built with the stock Android SDK, driven headless.

use idorid_ffi::App;

fn load(path: &str) -> App {
    let p = format!("{}/../../{path}", env!("CARGO_MANIFEST_DIR"));
    App::load(
        std::fs::read(&p).unwrap_or_else(|e| panic!("{p}: {e}")),
        false,
    )
    .expect("load")
}

fn ui(json: &str) -> serde_json::Value {
    let v: serde_json::Value = serde_json::from_str(json).expect("valid json");
    assert!(v["error"].is_null(), "app error: {}", v["error"]);
    v
}

/// Depth-first search for the first view matching `pred`.
fn find<'a>(
    v: &'a serde_json::Value,
    pred: &dyn Fn(&serde_json::Value) -> bool,
) -> Option<&'a serde_json::Value> {
    if pred(v) {
        return Some(v);
    }
    v["children"].as_array()?.iter().find_map(|c| find(c, pred))
}

fn by_text<'a>(v: &'a serde_json::Value, text: &str) -> &'a serde_json::Value {
    find(&v["root"], &|n| n["text"].as_str() == Some(text))
        .unwrap_or_else(|| panic!("no view with text {text:?} in {v}"))
}

fn by_type<'a>(v: &'a serde_json::Value, ty: &str) -> &'a serde_json::Value {
    find(&v["root"], &|n| n["type"].as_str() == Some(ty))
        .unwrap_or_else(|| panic!("no {ty} in {v}"))
}

fn id(v: &serde_json::Value) -> i32 {
    v["id"].as_i64().unwrap() as i32
}

#[test]
fn sample_app_counts_taps() {
    let mut app = load("tests/fixtures/sample.apk");
    assert_eq!(app.label, "Idorid Sample");
    let v = ui(&app.launch());
    by_text(&v, "Hello from Android");
    let field = id(by_type(&v, "EditText"));
    let button = id(by_text(&v, "Tap"));
    ui(&app.set_text(field, "Kim"));
    ui(&app.click(button));
    let v = ui(&app.click(button));
    by_text(&v, "Tapped 2, Kim");
    assert!(app.take_log().contains("I/Sample: Tapped 2, Kim"));
    let v = ui(&app.back());
    assert_eq!(v["finished"], true);
}

#[test]
fn todo_app_full_flow() {
    let mut app = load("examples/todo.apk");
    let v = ui(&app.launch());
    by_text(&v, "2 open · running for 0s");
    // add a task
    ui(&app.set_text(id(by_type(&v, "EditText")), "Buy milk"));
    let v = ui(&app.click(id(by_text(&v, "Add"))));
    by_text(&v, "• Buy milk");
    assert_eq!(v["toasts"][0], "Added \"Buy milk\"");
    // complete it through the dialog
    let v = ui(&app.click(id(by_text(&v, "• Buy milk"))));
    assert_eq!(v["dialog"]["title"], "Done?");
    let v = ui(&app.click(-1));
    assert!(v.get("dialog").is_none());
    by_text(&v, "2 open · running for 0s");
    // filter
    let v = ui(&app.click(id(by_type(&v, "CheckBox"))));
    assert!(find(&v["root"], &|n| n["text"].as_str()
        == Some("• Star the repo"))
    .is_none());
    // second activity with extras
    let v = ui(&app.click(id(by_text(&v, "About"))));
    assert_eq!(v["title"], "About");
    by_text(&v, "Open tasks: 2\nAndroid API 30 on idorid");
    let logo = by_type(&v, "ImageView");
    let png = app
        .resource_bytes(logo["src"].as_u64().unwrap() as u32)
        .expect("logo bytes");
    assert!(png.starts_with(b"\x89PNG"));
    assert!(app.vm.image_sizer.as_ref().unwrap()(logo["src"].as_u64().unwrap() as u32).is_some());
    let v = ui(&app.back());
    by_text(&v, "Todo (Android)");
    // delayed handler messages report when to tick next
    assert!(v["next"].as_i64().unwrap() >= 0);
}

#[test]
fn paint_canvas_touch_and_clear() {
    let mut app = load("examples/paint.apk");
    let v = ui(&app.launch());
    let canvas = find(&v["root"], &|n| n.get("draw").is_some()).expect("canvas view");
    assert_eq!(canvas["touch"], true);
    // layout gives the canvas its size on the first frame, so it draws immediately
    assert!(canvas["frame"][2].as_i64().unwrap() > 0);
    assert!(!canvas["draw"].as_array().unwrap().is_empty());
    let canvas_id = id(canvas);
    let clear_id = id(by_text(&v, "Clear"));
    let v = ui(&app.set_size(canvas_id, 320, 480));
    assert_eq!(v["next"], 16);
    let canvas = find(&v["root"], &|n| id(n) == canvas_id).unwrap();
    let ops = canvas["draw"].as_array().unwrap();
    for op in ["color", "oval", "rect", "rrect", "path", "text", "line"] {
        assert!(
            ops.iter().any(|item| item[0] == op),
            "missing {op}: {ops:?}"
        );
    }
    ui(&app.touch(canvas_id, 0, 20.5, 30.0));
    ui(&app.touch(canvas_id, 2, 40.0, 50.0));
    let v = ui(&app.touch(canvas_id, 1, 60.0, 70.0));
    let canvas = find(&v["root"], &|n| id(n) == canvas_id).unwrap();
    let path = canvas["draw"]
        .as_array()
        .unwrap()
        .iter()
        .find(|op| op[0] == "path")
        .unwrap();
    assert!(path[1].as_str().unwrap().contains("L 60 70"));
    let v = ui(&app.click(clear_id));
    let canvas = find(&v["root"], &|n| id(n) == canvas_id).unwrap();
    let path = canvas["draw"]
        .as_array()
        .unwrap()
        .iter()
        .find(|op| op[0] == "path")
        .unwrap();
    assert_eq!(path[1], "");
}

/// Kotlin app (kotlin-stdlib dexed in): lambdas, sealed classes, BigDecimal,
/// and a weighted keypad laid out by the Java layout pass.
#[test]
fn kotlin_calculator() {
    let mut app = load("examples/calc.apk");
    let v = ui(&app.launch());
    let key = |v: &serde_json::Value, label: &str| -> i32 { id(by_text(v, label)) };
    // the five keypad rows split the width evenly
    let seven = by_text(&v, "7");
    let eight = by_text(&v, "8");
    assert_eq!(seven["frame"][2], eight["frame"][2]);
    assert!(seven["frame"][2].as_i64().unwrap() > 60);
    for label in ["1", "+", "2", "×", "3", "="] {
        let k = key(&v, label);
        ui(&app.click(k));
    }
    let v = ui(&app.ui());
    by_text(&v, "1 + 2 × 3 = 7");
    for label in ["AC", "1", "÷", "3", "="] {
        let k = key(&v, label);
        ui(&app.click(k));
    }
    let v = ui(&app.ui());
    by_text(&v, "0.333333333333");
}
