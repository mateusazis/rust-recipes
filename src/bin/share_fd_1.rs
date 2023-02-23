use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::os::unix::net::{SocketAncillary, UnixListener, UnixStream};

fn main_internal() -> Result<(), Box<dyn Error>> {
    let arg: String = std::env::args().last().unwrap();
    let f = File::open(arg.as_str())?;

    let listener = UnixListener::bind("/tmp/my_unix_socket")?;
    loop {
        let (stream, _addr) = listener.accept()?;
        // stream.write(buf)
    }
}

fn main() {
    main_internal().expect("should suceed");
}
