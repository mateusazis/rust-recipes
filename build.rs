use std::fmt::format;

pub fn main() {
    let mut done =false;

    if let Ok(target) = std::env::var("TARGET") {
        if target == "aarch64-unknown-linux-gnu" {
            let output = std::process::Command::new("make")
            .args(["libmylib.so"])
            .env("CFLAGS", "--target=arm64v7a-linux-gnueabi -fuse-ld=lld")
            .current_dir("./src/util")
            .output()
            .expect("should have ran Make");
            done = true;
            // panic!("Output: out: {}, err: {}", String::from_utf8(output.stdout).unwrap(), String::from_utf8(output.stderr).unwrap());
        }
        // panic!("failed w/ target: {}", target);
    }
    // panic!("failed w/o target, vars: {}", std::env::vars().map(|k| format!("{} - {},", k, v)));
    // panic!("the linker is {}", std::env::var("CARGO_TARGET_AARCH64-UNKNOWN_LINUX_MUSL_LINKER").unwrap());
     
    if !done {
    std::process::Command::new("make")
        .args(["libmylib.so"])
        .current_dir("./src/util")
        // .env("CFLAGS", "-fuse-ld=lld")
            // .env("CFLAGS", "--target=arm64v7a-linux-gnueabi -fuse-ld=lld")
        .output()
        .expect("should have ran Make");
    }

    println!("cargo:rustc-link-search=./src/util");
    println!("cargo:rustc-link-lib=mylib");
}
