use std::{fs, io};
use std::os::unix::net::UnixDatagram;
use std::time::{SystemTime, UNIX_EPOCH};
use rlibbencode::variables::bencode_object::BencodeObject;
use rlibbencode::variables::inter::bencode_variable::{FromBencode, ToBencode};

const UNIX_RPC_PATH: &str = "/tmp/find9.sock";

pub fn send(bencode: BencodeObject) -> io::Result<BencodeObject> {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();

    let path = format!("/tmp/find9_{}.sock", unique);

    println!("{:?}", String::from_utf8(bencode.to_bencode()));

    let socket = UnixDatagram::bind(&path)?;
    socket.send_to(&bencode.to_bencode(), UNIX_RPC_PATH)?;

    let mut buf = [0u8; 65535];
    socket.recv(&mut buf)?;

    fs::remove_file(&path)?;

    BencodeObject::from_bencode(&buf)
}
