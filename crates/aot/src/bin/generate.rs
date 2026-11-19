#[path = "../generator.rs"]
mod generator;
fn main() -> std::io::Result<()> {
    let args: Vec<_> = std::env::args_os().collect();
    if args.len() != 5 {
        eprintln!("usage: generate PROFILE ROM OUTPUT LIMIT");
        std::process::exit(2);
    }
    let mut out = std::fs::File::create(&args[3])?;
    let n = generator::generate(
        std::path::Path::new(&args[1]),
        std::path::Path::new(&args[2]),
        args[4].to_string_lossy().parse().expect("limit"),
        &mut out,
    )?;
    eprintln!("generated {n} pages");
    Ok(())
}
