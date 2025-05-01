use std::io;
use crate::commands::{record, user};

pub fn command(args: &[String]) -> io::Result<()> {
    match args.first().unwrap().as_str() {
        "record" => {
            record::command(&args[1..])?;
        }
        "user" => {
            user::command(&args[1..])?;
        }
        "unblock" | "block" => {

        }
        "help" | "-h" => {
            println!("{}\r\n", commands());
            println!("{}\r\n", record::commands());
            println!("{}\r\n", user::commands());
            println!("{}\r\n", arguments());
            println!("{}\r\n", record::arguments());
            println!("{}\r\n", user::arguments());
            println!("Run 'find9 COMMAND -h' for more information on a command.")
        }
        "version" | "-v" => {
            println!("find9 version {}", env!("CARGO_PKG_VERSION"));
        }
        _ => {}
    }

    Ok(())
}

pub fn commands() -> String {
    String::from("General Commands:\r\n  \
    record\t\tAdd, Get, Remove DNS records\r\n  \
    user\t\t\tCreate, Edit, Delete user\r\n  \
    block\t\t\tBlock IP Address\r\n  \
    unblock\t\tUnblock IP Address\r\n  \
    help\t\t\tGet a list of commands\r\n  \
    version\t\tGet the version of find9")
}

pub fn arguments() -> String {
    String::from("General Options:\r\n  \
    -h\t\t\tGet a list of commands\r\n  \
    -v\t\t\tGet the version of find9")
}
