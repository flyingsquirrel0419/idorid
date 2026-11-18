//! Runs tests/oracle.sh (JVM vs idorid differential tests) when a JDK and the
//! Android build tools are available.

#[test]
fn oracle() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");
    let have_java = std::process::Command::new("javac")
        .arg("-version")
        .output()
        .is_ok();
    let sdk = std::env::var("ANDROID_HOME")
        .unwrap_or_else(|_| format!("{}/android-sdk", std::env::var("HOME").unwrap_or_default()));
    if !have_java || !std::path::Path::new(&sdk).join("build-tools").exists() {
        eprintln!("skipping oracle tests: need javac and Android build-tools");
        return;
    }
    let st = std::process::Command::new("bash")
        .arg(format!("{root}/tests/oracle.sh"))
        .status()
        .expect("run oracle.sh");
    assert!(st.success(), "oracle tests failed");
}
