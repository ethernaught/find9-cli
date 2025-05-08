mod commands;
mod utils;

use std::{env, io};
use crate::commands::general::command;

//record create -r a -c in -name uncentralized.unet -address 127.0.0.1 -ttl 300 -local true

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        return command(&[String::from("-h")]);
    }

    command(&args)
}
