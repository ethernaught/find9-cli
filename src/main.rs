mod commands;

use std::{env, io};
use std::io::ErrorKind;
use crate::commands::record::record_commands;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        return Err(io::Error::new(ErrorKind::NotFound, "Missing arguments"));
    }

    match args.first().unwrap().as_str() {
        "record" => {
            record_commands(&args[1..])?;
        }
        "help" | "-h" => {
            //HELP COMMAND
        }
        _ => {}
    }

    //UNIX-STREAM WILL HANDLE

    /*
    TYPES OF ARGS

    -h = HELP

    -r = RECORD TYPE
    -c = CLASS NAME
    -domain = DOMAIN
    -address = IP-ADDRESS
    -ttl = TTL
    -cache_flush = CACHE FLUSH - BOOLEAN
    -local = LAN ONLY - default is any
    -external = EXTERNAL ONLY - default is any

    EXAMPLE
    record add -r a -c in -domain net.unet -address 127.0.0.1 -ttl 300 -cache_flush true -local true
    record remove -r a -c in -domain net.unet -address 127.0.0.1
    */

    //let stream = UnixStream::connect("/tmp/find9.sock").unwrap();



    Ok(())
}
