use std::{net::{TcpStream, ToSocketAddrs}, fmt::{Display}, io::{Write, Read}};
use rand::random;

const MAGIC_STRING : &'static str = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11";
struct WebSocket {
    stream: TcpStream
}

impl WebSocket {
    pub fn connect (addr: impl ToSocketAddrs, path: impl Display) -> std::io::Result<Self> {
        let key = random::<[u8;16]>();
        let base64_key = base64::encode(&key);
        let mut stream = TcpStream::connect(addr)?;

        /*panic!(
            "GET {path} HTTP/1.1\r\nHost: {}\r\nOrigin: {}\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Key: {base64_key}\r\nSec-WebSocket-Version: 13\r\n",
            stream.peer_addr()?,
            stream.local_addr()?
        );*/

        // REQUEST
        stream.write_fmt(format_args!(
            "GET {path} HTTP/1.1\r\nHost: {}\r\nOrigin: {}\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Key: {base64_key}\r\nSec-WebSocket-Version: 13\r\n",
            stream.peer_addr()?,
            stream.local_addr()?
        ))?;
        stream.flush()?;

        // RESPONSE
        let mut response = String::with_capacity(1024);
        stream.read_to_string(&mut response)?;

        println!("{response}");
        todo!()
    }
}

#[test]
fn connect () {
    let id = random::<u16>();
    let connect = WebSocket::connect("127.0.0.1:8080", format!("/test/player/conn/{id}")).unwrap();
}