#![feature(unix_socket_ancillary_data)]

use std::error::Error;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader, IoSliceMut, Read};
use std::os::fd::{AsRawFd, FromRawFd};
#[cfg(target_os = "linux")]
use std::os::unix::net::{AncillaryData, SocketAncillary, UnixStream};

fn prompt(msg: &str) -> Result<(), std::io::Error> {
    let mut line = String::new();
    println!("{}", msg);
    stdin().lock().read_line(&mut line)?;
    Ok(())
}

#[cfg(target_os = "linux")]
fn main_internal() -> Result<(), Box<dyn Error>> {
    let arg: String = std::env::args().last().unwrap();
    let f = File::open(arg.as_str())?;

    println!("Connecting to stream...");
    let stream = UnixStream::connect("/tmp/my_unix_socket")?;
    let mut buf1 = [0u8; 128];
    let mut sli = [IoSliceMut::new(&mut buf1); 1];
    let mut buf2 = [0u8; 128];
    let mut ancillary = SocketAncillary::new(&mut buf2);
    stream.recv_vectored_with_ancillary(&mut sli, &mut ancillary)?;

    let msgs = ancillary.messages();
    for msg in msgs {
        println!("msg ");
        let data = msg.expect("get ancillary data");
        if let AncillaryData::ScmRights(rights) = data {
            for fd in rights {
                let f = unsafe { File::from_raw_fd(fd.as_raw_fd()) };
                let mut r = BufReader::new(f);
                let mut content = Vec::new();

                prompt("about to read file...")?;
                r.read_to_end(&mut content)?;
                let content = String::from_utf8(content)?;
                println!("Contents of the file: '{}'", content);
            }
        }
    }
    prompt("about to quit...")?;
    Ok(())
}

fn main() {
    #[cfg(target_os = "linux")]
    main_internal().expect("should suceed");
}
