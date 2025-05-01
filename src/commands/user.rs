use std::io;

pub fn command(args: &[String]) -> io::Result<()> {
    match args.first().unwrap().as_str() {
        _ => {}
    }

    Ok(())
}

pub fn commands() -> String {
    String::from("User Commands:\r\n  \
    create\t\tCreate a user\r\n  \
    edit\t\t\tEdit a user\r\n  \
    remove\t\tRemove a user")
}

pub fn arguments() -> String {
    String::from("User Options:")
}
