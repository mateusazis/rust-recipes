#![feature(unix_socket_ancillary_data)]

use std::error::Error;
use std::fs::File;
use std::io::IoSlice;
use std::os::fd::AsRawFd;
use std::os::unix::net::{SocketAncillary, UnixListener};

fn get_file_name() -> String {
    if std::env::args().len() >= 2 {
        return std::env::args().last().unwrap();
    }
    String::from("/home/azis/windows_dev/rustplay/recipes/src/bin/lorem.txt")
}

fn main_internal() -> Result<(), Box<dyn Error>> {
    let f = File::open(get_file_name())?;
    let path = "/tmp/my_unix_socket";
    std::fs::remove_file(path)?;

    let listener = UnixListener::bind(path)?;
    loop {
        println!("Waiting for client...");
        let (stream, _addr) = listener.accept()?;

        let buf1 = [0u8; 128];
        let msgs = [IoSlice::new(&buf1); 1];
        let mut buf2 = [0u8; 128];
        let mut anc = SocketAncillary::new(&mut buf2);
        anc.add_fds(&[f.as_raw_fd()]);

        stream.send_vectored_with_ancillary(&msgs, &mut anc)?;
    }
}

fn main() {
    main_internal().expect("should suceed");
}
