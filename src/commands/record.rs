use std::io;

pub fn record_commands(args: &[String]) -> io::Result<()> {
    match args.first().unwrap().as_str() {
        "add" => {
            println!("Adding");
            args_to_record(&args[1..])?;
        }
        "get" => {

        }
        "remove" => {

        }
        _ => {}
    }

    Ok(())
}

fn args_to_record(args: &[String]) -> io::Result<()> {
    for pair in args.chunks(2) {
        if pair.len() == 2 {
            let key = &pair[0];
            let value = &pair[1];
            println!("{key} = {value}");

        } else {
            eprintln!("Warning: key `{}` has no corresponding value", pair[0]);
        }
    }

    Ok(())
}
