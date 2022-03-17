use std::{net::{TcpStream, ToSocketAddrs}, fmt::{Display}};
use rand::random;

const MAGIC_STRING : &'static str = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11";
struct WebSocket {
    stream: TcpStream
}

impl WebSocket {
    pub fn connect (addr: impl ToSocketAddrs, path: impl Display) -> std::io::Result<Self> {
        let key = base64::encode(random::<u16>().to_ne_bytes());
        let stream = TcpStream::connect(addr)?;
        
        let request = format!(
            "GET {path} HTTP/1.1\nHost:{}\nUpgrade:websocket\nConnection:Upgrade\nSec-WebSocket-Key:{key}\nSec-WebSocket-Version:13",
            stream.peer_addr()?
        );

        println!("{request}");
        todo!()
    }
}

#[test]
fn connect () {
    let connect = WebSocket::connect("127.0.0.1", "/player/conn");
    let id = random::<u64>();
}