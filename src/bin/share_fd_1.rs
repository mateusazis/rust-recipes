#![feature(unix_socket_ancillary_data)]

use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::os::fd::AsRawFd;
use std::os::unix::net::{SocketAncillary, UnixListener};

fn main_internal() -> Result<(), Box<dyn Error>> {
    let arg: String = std::env::args().last().unwrap();
    let f = File::open(arg.as_str())?;

    let listener = UnixListener::bind("/tmp/my_unix_socket")?;
    loop {
        println!("Waiting for data...");
        let (mut stream, _addr) = listener.accept()?;

        let mut buf = [0u8; 1];
        let mut anc = SocketAncillary::new(&mut buf);
        anc.add_fds(&[f.as_raw_fd()]);

        stream.write(&buf)?;
        // stream.write(buf)
    }
}

fn main() {
    main_internal().expect("should suceed");
}
