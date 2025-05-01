use std::{fs, io};
use std::os::unix::net::UnixDatagram;
use std::time::{SystemTime, UNIX_EPOCH};
use rlibbencode::variables::bencode_object::BencodeObject;
use rlibbencode::variables::inter::bencode_variable::BencodeVariable;

const UNIX_RPC_PATH: &str = "/tmp/find9.sock";

pub fn send(bencode: BencodeObject) -> io::Result<BencodeObject> {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();

    let path = format!("/tmp/find9_{}.sock", unique);

    let socket = UnixDatagram::bind(&path)?;
    socket.send_to(&bencode.encode(), UNIX_RPC_PATH)?;

    let mut buf = [0u8; 65535];
    socket.recv(&mut buf)?;

    fs::remove_file(&path)?;

    BencodeObject::decode(&buf)
}
