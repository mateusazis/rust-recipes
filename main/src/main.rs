#![feature(thread_id_value, portable_simd)]
mod util;

// Mac instructions: set DYLD_FALLBACK_LIBRARY_PATH

struct RunOption<'a> {
    name: &'a str,
    func: fn(),
}

impl<'a> RunOption<'a> {
    fn new(name: &'a str, func: fn()) -> RunOption {
        RunOption { name, func }
    }
}

fn main_aarch64() {
    let options = vec![
        RunOption::new("FFI", util::ffi::main),
        RunOption::new("GLIBC", util::glibc::main),
        RunOption::new("Inline Assembly", util::inline_assembly::main),
        RunOption::new("Threading", util::threading::main),
        RunOption::new("Async", util::asyncer::main),
        RunOption::new("Manual async", util::manual_async::main),
    ];

    println!("Choose an option:");
    for i in 0..options.len() {
        println!("{}) {}", i, options.get(i).unwrap().name)
    }
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf = buf.replace("\n", "");
    let chosen: usize = buf.parse().unwrap();
    let o = options.get(chosen).unwrap();
    (o.func)();
}

fn main() {
    if cfg!(target_arch = "aarch64") && cfg!(target_os = "linux") {
        main_aarch64();
        return;
    }

    util::dynamic_dispatch::main()
}
