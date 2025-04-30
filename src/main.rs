use std::os::unix::net::UnixStream;

fn main() {
    //TAKE ARGS - RUN THROUGH UNIX-STREAM
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
    add -r a -c in -domain net.unet -address 127.0.0.1 -ttl 300 -cache_flush = true -local true
    remove -r a -c in -domain net.unet -address 127.0.0.1
    */

    let stream = UnixStream::connect("/tmp/find9.sock").unwrap();


}
