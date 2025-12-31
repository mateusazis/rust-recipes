#![cfg_attr(target_os = "linux", feature(unix_socket_ancillary_data))]

pub mod other_bin_utils;

#[cfg(target_os = "linux")]
mod internal {

    use other_bin_utils::err_with_backtrace::add_stack_trace;
    use std::error::Error;
    use std::fs::File;
    use std::io::IoSlice;
    use std::os::fd::AsRawFd;
    use std::os::unix::net::{SocketAncillary, UnixListener};

    fn get_file_name() -> String {
        if std::env::args().len() >= 2 {
            return std::env::args().last().unwrap();
        }
        String::from("/home/azis/windows_dev/rustplay/recipes2/src/bin/lorem.txt")
    }

    pub fn main_internal() -> Result<(), Box<dyn Error>> {
        let f = add_stack_trace(File::open(get_file_name()))?;
        let path = "/tmp/my_unix_socket";
        // don´t care if this fails
        add_stack_trace(std::fs::remove_file(path)).unwrap_or(());

        let listener = add_stack_trace(UnixListener::bind(path))?;
        loop {
            println!("Waiting for client...");
            let (stream, _addr) = listener.accept()?;

            let data_buf: [u8; 1] = [0u8; 1];
            let data_io_slice = IoSlice::new(&data_buf);

            let mut ancillary_buf = [0u8; 128];
            let mut anccillary = SocketAncillary::new(&mut ancillary_buf);
            anccillary.add_fds(&[f.as_raw_fd()]);

            add_stack_trace(stream.send_vectored_with_ancillary(
                std::slice::from_ref(&data_io_slice),
                &mut anccillary,
            ))?;
        }
    }
}

#[cfg(not(target_os = "linux"))]
mod internal {
  use std::error::Error;

  pub fn main_internal() -> Result<(), Box<dyn Error>> {
    Ok(())
  }
}

fn main() {
    internal::main_internal().expect("should suceed");
}
