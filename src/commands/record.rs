use std::collections::HashMap;
use std::io;
use std::net::IpAddr;
use std::os::unix::net::{UnixDatagram, UnixStream};
use std::str::FromStr;
use rlibbencode::variables::bencode_object::{BencodeObject, PutObject};
use rlibbencode::variables::inter::bencode_variable::BencodeVariable;

pub fn command(args: &[String]) -> io::Result<()> {
    match args.first().unwrap().as_str() {
        "add" | "get" | "remove" => {
            let mut bencode = BencodeObject::new();
            bencode.put("v", env!("CARGO_PKG_VERSION"));
            bencode.put("t", args.first().unwrap().as_str());
            bencode.put("q", args_to_record(&args[1..])?);

            println!("{}", bencode.to_string());

            let socket = UnixDatagram::bind("/var/run/find9.sock")?;
            socket.send(&bencode.encode())?;

            let mut buf = [0u8; 65535];
            socket.recv(&mut buf)?;

            let bencode = BencodeObject::decode(&buf)?;
            println!("{}", bencode.to_string());
        }
        "help" | "-h" => {
            println!("{}\r\n", commands());
            println!("{}", arguments());
            return Ok(());
        }
        _ => return Err(io::Error::new(io::ErrorKind::NotFound, "Invalid argument")),
    }

    Ok(())
}

pub fn commands() -> String {
    String::from("Record Commands:\r\n  \
    add\t\t\tAdd a record to the DNS table\r\n  \
    get\t\t\tGet a record from the DNS table\r\n  \
    remove\t\tRemove a record from the DNS table")
}

pub fn arguments() -> String {
    String::from("Record Options:\r\n  \
    -r\t\t\tRecord Type\r\n  \
    -c\t\t\tDNS Class Type\r\n  \
    -domain\t\tDomain name\r\n  \
    -address\t\tIP Address\r\n  \
    -ttl\t\t\tTTL (Time To Live)\r\n  \
    -cache_flush\t\tCache flush toggle (true/false)\r\n  \
    -local\t\tLAN only toggle (true/false)\r\n  \
    -external\t\tExternal only toggle (true/false)")
}

fn args_to_record(args: &[String]) -> io::Result<BencodeObject> {
    let mut bencode = BencodeObject::new();

    let arg_to_key: HashMap<&str, ArgMeta> = HashMap::from([
        ("-r", ArgMeta::new("record", ArgTypes::Bytes)),
        ("-c", ArgMeta::new("class", ArgTypes::Bytes)),
        ("-domain", ArgMeta::new("domain", ArgTypes::Bytes)),
        ("-address", ArgMeta::new("address", ArgTypes::Address)),
        ("-ttl", ArgMeta::new("ttl", ArgTypes::Number)),
        ("-cache_flush", ArgMeta::new("cache_flush", ArgTypes::Bool)),
        ("-local", ArgMeta::new("local", ArgTypes::Bool))
    ]);

    for pair in args.chunks(2) {
        if pair.len() == 2 {
            if let Some(&ref meta) = arg_to_key.get(pair[0].as_str()) {
                match meta._type {
                    ArgTypes::Bytes => {
                        bencode.put(&meta.name, pair[1].as_str());
                    }
                    ArgTypes::Number => {
                        bencode.put(&meta.name, pair[1].parse::<u128>().unwrap());
                    }
                    ArgTypes::Address => {
                        let ip = IpAddr::from_str(pair[1].as_str()).unwrap();
                        match ip {
                            IpAddr::V4(ip) => {
                                bencode.put(&meta.name, u32::from(ip));
                            }
                            IpAddr::V6(ip) => {
                                bencode.put(&meta.name, u128::from(ip));
                            }
                        }
                    }
                    ArgTypes::Bool => {
                        bencode.put(&meta.name, pair[1].parse::<bool>().unwrap() as u8);
                    }
                    _ => !unreachable!()
                }

            } else {
                eprintln!("Warning: key `{}` is not a valid", pair[0]);
            }

        } else {
            eprintln!("Warning: key `{}` has no corresponding value", pair[0]);
        }
    }

    Ok(bencode)
}

enum ArgTypes {
    Bytes,
    Number,
    Address,
    Bool
}

struct ArgMeta {
    name: String,
    _type: ArgTypes
}

impl ArgMeta {

    fn new(name: &str, _type: ArgTypes) -> Self {
        Self {
            name: name.to_string(),
            _type
        }
    }
}
