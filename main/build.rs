use std::io::BufRead;
use std::path::Path;

pub fn main() {
    let mut cmd: std::process::Command = std::process::Command::new("make");
    cmd.args(["-B", "libmylib.so"]).current_dir("./src/util");

    let mut c_flags = String::new();

    let target = std::env::var("TARGET").unwrap();
    c_flags.push_str("-g --target=");
    c_flags.push_str(target.as_str());

    let os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    match os.as_str() {
        "linux" => c_flags.push_str(" -fuse-ld=lld"),
        "macos" => c_flags.push_str(" -lSystem"),
        _ => {}
    };

    cmd.env("CFLAGS", c_flags.as_str());

    let output = cmd.output().expect("should have ran Make");

    let code = output.status.code().unwrap();
    if code != 0 {
        let s: std::vec::Vec<String> = output.stderr.lines().map(|line| line.unwrap()).collect();
        panic!(
            "make should have exited with 0, but was {}. Output:\n{:?}",
            code, s
        );
    }

    for line in output.stdout.lines() {
        println!("{}", line.unwrap());
    }

    let curr_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let curr_dir = Path::new(curr_dir.as_str());
    let library_search_path = curr_dir.join("src/util");

    println!(
        "cargo:rustc-link-search={}",
        library_search_path.to_str().unwrap()
    );
    println!("cargo:rustc-link-lib=mylib");
}
