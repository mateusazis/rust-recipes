use std::process::exit;

pub fn main() {
    unsafe {
        libc::printf(
            "Parent: My pid is %d and my parent is %d\n\0".as_ptr() as *const i8,
            libc::getpid(),
            libc::getppid(),
        );
        let child_pid = libc::fork();
        match child_pid {
            0 => {
                println!("Child {}: sleeping for 2 seconds...", libc::getpid());
                libc::sleep(2);
                exit(42);
            }
            _ => {
                println!(
                    "Parent {}: waiting for child '{}'...",
                    libc::getpid(),
                    child_pid
                );

                let mut result: i32 = 0;
                let ptr: *mut i32 = &mut result as *mut i32;
                libc::waitpid(child_pid, ptr, 0);
                println!(
                    "Parent {}: child {} exited with code {}",
                    libc::getpid(),
                    child_pid,
                    libc::WEXITSTATUS(result)
                );
            }
        };
    };
}
