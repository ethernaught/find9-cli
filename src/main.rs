mod commands;

use std::{env, io};
use std::io::ErrorKind;
use crate::commands::general::command;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        return Err(io::Error::new(ErrorKind::NotFound, "Missing arguments"));
    }

    command(&args)?;

    //let stream = UnixStream::connect("/tmp/find9.sock").unwrap();



    Ok(())
}
