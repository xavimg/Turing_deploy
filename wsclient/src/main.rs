extern crate websocket;
use websocket::{ClientBuilder, Message, header::{Headers}};

pub static TOKEN : &'static str = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VyX2lkIjoxMiwiZXhwIjoxNjQ2NjU5NTc3LCJpYXQiOjE2NDY0MDAzNzcsImlzcyI6IlRlc3RpbmcifQ.gv3Rd--jB4sT2d4PSNfYAivbFXQpqexi7EH5YSpaAtU";

fn main () {
    let mut headers = Headers::new();
    headers.append_raw("Authorization", format!("Bearer {TOKEN}").into_bytes());

    let mut client = ClientBuilder::new("ws://localhost:8080/player/conn")
        .unwrap()
        .custom_headers(&headers)
        .connect_insecure()
        .unwrap();

    let message = Message::text("Hello, World!");
    client.send_message(&message).unwrap(); // Send message
    println!("{:?}", client.recv_message().unwrap());
}