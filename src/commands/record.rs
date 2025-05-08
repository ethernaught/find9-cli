use std::collections::HashMap;
use std::io;
use std::net::IpAddr;
use std::str::FromStr;
use rlibbencode::bencode;
use rlibbencode::variables::bencode_number::BencodeNumber;
use rlibbencode::variables::bencode_object::{BencodeObject, GetObject, PutObject};
use rlibbencode::variables::inter::bencode_variable::BencodeVariable;
use crate::utils::unix_rpc::send;

pub fn command(args: &[String]) -> io::Result<()> {
    match args.first().unwrap().as_str() {
        "create" | "get" | "remove" => {
            //let mut bencode = BencodeObject::new();
            //bencode.put("v", env!("CARGO_PKG_VERSION"));
            //bencode.put("t", args.first().unwrap().as_str());
            //bencode.put("q", args_to_record(&args[1..])?);

            let v = env!("CARGO_PKG_VERSION");
            let t = args.first().unwrap().as_str();
            let q = args_to_record(&args[1..])?;

            let bencode = bencode!({
                "v": v,
                "t": t,
                "q": q
            });
            //println!("{}", bencode);

            let bencode = send(bencode)?;
            match bencode.get::<BencodeNumber>("s").ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Status not found"))?.parse::<u16>().unwrap() {
                0 => {}
                (s) => {
                    println!("Error: status: {}", s);
                }
            }
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
    create\t\tCreate a record to the DNS table\r\n  \
    get\t\t\tGet a record from the DNS table\r\n  \
    remove\t\tRemove a record from the DNS table")
}

pub fn arguments() -> String {
    String::from("Record Options:\r\n  \
    -r\t\t\tRecord Type\r\n  \
    -c\t\t\tDNS Class Type\r\n  \
    -ttl\t\t\tTTL (Time To Live)\r\n  \
    -name\t\t\tName\r\n  \
    -address\t\tIP Address\r\n  \
    -flags\t\tFlags\r\n  \
    -protocol\t\tProtocol\r\n  \
    -algorithm\t\tAlgorithm\r\n  \
    -public_key\t\tPublic Key in Base64\r\n  \
    -priority\t\tPriority\r\n  \
    -server\t\tServer\r\n  \
    -domain\t\tDomain\r\n  \
    -weight\t\tWeight\r\n  \
    -port\t\t\tPort\r\n  \
    -content\t\tContent\r\n  \
    -local\t\tLAN only toggle (true/false)\r\n  \
    -external\t\tExternal only toggle (true/false)")
}

fn args_to_record(args: &[String]) -> io::Result<BencodeObject> {
    let mut bencode = BencodeObject::new();

    let arg_to_key: HashMap<&str, ArgMeta> = HashMap::from([
        ("-r", ArgMeta::new("record", ArgTypes::Bytes)),
        ("-c", ArgMeta::new("class", ArgTypes::Bytes)),
        ("-name", ArgMeta::new("name", ArgTypes::Bytes)),
        ("-ttl", ArgMeta::new("ttl", ArgTypes::Number)),
        ("-address", ArgMeta::new("address", ArgTypes::Address)),
        ("-target", ArgMeta::new("target", ArgTypes::Bytes)),
        ("-flags", ArgMeta::new("flags", ArgTypes::Number)),
        ("-protocol", ArgMeta::new("protocol", ArgTypes::Number)),
        ("-algorithm", ArgMeta::new("algorithm", ArgTypes::Number)),
        ("-public_key", ArgMeta::new("public_key", ArgTypes::Bytes)),
        ("-priority", ArgMeta::new("priority", ArgTypes::Number)),
        ("-server", ArgMeta::new("server", ArgTypes::Bytes)),
        ("-domain", ArgMeta::new("domain", ArgTypes::Bytes)),
        ("-weight", ArgMeta::new("weight", ArgTypes::Number)),
        ("-port", ArgMeta::new("port", ArgTypes::Number)),
        ("-content", ArgMeta::new("content", ArgTypes::Bytes)),
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
