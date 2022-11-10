use std::io::BufRead;

pub fn main() {
    let mut cmd: std::process::Command = std::process::Command::new("make");
    cmd.args(["-B", "libmylib.so"]).current_dir("./src/util");

    if let Ok(target) = std::env::var("TARGET") {
        if target.starts_with("aarch64-unknown-linux-") {
            cmd.env("CFLAGS", "--target=arm64v7a-linux-gnueabi -fuse-ld=lld -g");
        }
    }
    let output = cmd.output().expect("should have ran Make");

    let code = output.status.code().unwrap();
    if code != 0 {
        let s : std::vec::Vec<String> = output.stderr.lines().map(|line| line.unwrap()).collect();
        panic!("should have exited with 0, but was {}: {:?}", code, s);
    }

    for line in output.stdout.lines() {
        println!("{}", line.unwrap());
    }

    println!("cargo:rustc-link-search=./src/util");
    println!("cargo:rustc-link-lib=mylib");
}
