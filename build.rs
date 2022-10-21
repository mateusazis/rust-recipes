pub fn main() {
    std::process::Command::new("make")
        .args(["libmylib.so"])
        .current_dir("./src/util")
        .output()
        .expect("should have ran Make");

    println!("cargo:rustc-link-search=./src/util");
}
