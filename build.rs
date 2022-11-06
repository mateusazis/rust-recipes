pub fn main() {
    let mut cmd: std::process::Command = std::process::Command::new("make");
    cmd.args(["-B", "libmylib.so"]).current_dir("./src/util");

    if let Ok(target) = std::env::var("TARGET") {
        if target.starts_with("aarch64-unknown-linux-") {
            cmd.env("CFLAGS", "--target=arm64v7a-linux-gnueabi -fuse-ld=lld");
        }
    }

    cmd.output().expect("should have ran Make");

    println!("cargo:rustc-link-search=./src/util");
    println!("cargo:rustc-link-lib=mylib");
}
