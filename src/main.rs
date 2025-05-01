mod commands;
mod utils;

use std::{env, io};
use std::io::ErrorKind;
use crate::commands::general::command;

//record add -r a -c in -domain net.unet -address 127.0.0.1 -ttl 300 -cache_flush true -local true

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        return Err(io::Error::new(ErrorKind::NotFound, "Missing arguments"));
    }

    command(&args)?;

    //let stream = UnixStream::connect("/tmp/find9.sock").unwrap();



    Ok(())
}
