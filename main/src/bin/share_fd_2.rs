#![feature(unix_socket_ancillary_data)]

use another_library::err_with_backtrace::add_stack_trace;
use std::error::Error;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader, IoSliceMut, Read, Seek};
use std::os::fd::{AsRawFd, FromRawFd, RawFd};
use std::os::unix::net::{AncillaryData, SocketAncillary, UnixStream};

fn prompt(msg: &str) -> Result<(), std::io::Error> {
    let mut line = String::new();
    println!("{}\nPress ENTER to continue...", msg);
    stdin().lock().read_line(&mut line)?;
    Ok(())
}

fn get_file_name(fd: RawFd) -> Result<String, std::io::Error> {
    let link = std::fs::read_link(format!("/proc/self/fd/{}", fd))?;
    Ok(String::from(link.to_str().unwrap()))
}

fn get_fds(ancillary: SocketAncillary) -> Vec<RawFd> {
    let msgs = ancillary.messages();
    msgs.into_iter()
        .flat_map(|msg| {
            println!("Got message from stream");
            let data = msg.expect("get ancillary data");
            if let AncillaryData::ScmRights(rights) = data {
                Vec::from_iter(rights.into_iter())
            } else {
                Vec::new()
            }
        })
        .collect()
}

fn main_internal() -> Result<(), Box<dyn Error>> {
    println!("Connecting to stream...");
    let stream = add_stack_trace(UnixStream::connect("/tmp/my_unix_socket"))?;
    let mut buf1 = [0u8; 128];
    let mut sli = [IoSliceMut::new(&mut buf1); 1];
    let mut buf2 = [0u8; 128];
    let mut ancillary = SocketAncillary::new(&mut buf2);
    println!("Reading ancillary message...");
    add_stack_trace(stream.recv_vectored_with_ancillary(&mut sli, &mut ancillary))?;

    let fds = get_fds(ancillary);
    prompt("Got all messages, about to close the client socket")?;
    std::mem::drop(stream);

    for fd in fds {
        let mut f = unsafe { File::from_raw_fd(fd.as_raw_fd()) };
        f.seek(std::io::SeekFrom::Start(0))?;

        let mut r = BufReader::new(f);
        let mut content = Vec::new();

        prompt(
            format!(
                "About to read file {}...",
                get_file_name(fd.as_raw_fd()).unwrap()
            )
            .as_str(),
        )?;

        r.read_to_end(&mut content)?;
        let content = String::from_utf8(content)?;
        println!("Contents of the file: '{}'", content);
    }
    prompt("about to quit...")?;
    Ok(())
}

fn main() {
    main_internal().expect("should suceed");
}
