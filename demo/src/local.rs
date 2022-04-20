use std::{io::ErrorKind, task::Poll};
use tokio::{net::TcpStream, io::AsyncWriteExt, sync::Mutex};
use bson::oid::ObjectId;
use llml::vec::{EucVecd2, EucVecf2};
use rand::random;
use serde::{Deserialize, Serialize};
use serde_json::json;
use slg::{generics::{Circle}, renderer::opengl::{OpenGl, GlInstance}, Threadly, RenderInstance};
use reqwest::get;
use tokio_tungstenite::{tungstenite::{handshake::client::Request, Message}, connect_async, WebSocketStream, MaybeTlsStream};
use futures::{StreamExt, stream::{SplitSink, SplitStream}, SinkExt};
use crate::{PlayerRequest, world_to_local, local_to_world};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerLocation {
    pub system: ObjectId,
    pub position: EucVecd2
}

pub struct PlayerConnection {
    //pub client: WebSocketStream<MaybeTlsStream<TcpStream>>,
    pub write: Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>,
    pub read: Mutex<SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>>,
    pub location: Mutex<PlayerLocation>,
    pub token: String,
    pub circle: Threadly<Circle<OpenGl>>
}

impl PlayerConnection {
    pub async fn new (window: Threadly<GlInstance>) -> std::io::Result<Self> {
        let id = random::<u16>();
        let token = get(format!("http://localhost:8080/test/player/{id}")).await.map_err(|e| std::io::Error::new(ErrorKind::Other, e))?
            .text().await.map_err(|e| std::io::Error::new(ErrorKind::Other, e))?;

        let client = Self::connect_with_token(&token).await
            .map_err(|e| std::io::Error::new(ErrorKind::Other, e))?;

        let player = Self::get_player_info(&token).await?;
        let mut window = window.write().unwrap();

        let circle = window.create_circle(world_to_local(player.location.position), 0.01, player.color)
            .map_err(|e| std::io::Error::new(ErrorKind::Other, e))?;
        

        let (write, read) = client.split();
        Ok(Self { write: Mutex::new(write), read: Mutex::new(read), token, circle, location: Mutex::new(player.location) })
    }

    pub async fn translate (&mut self, delta: EucVecf2) -> std::io::Result<()> {
        let world = local_to_world(delta);

        let mut lock = self.location.lock().await;
        lock.position += world;

        let mut circle = self.circle.write().unwrap();
        circle.position += delta;
        
        let body = json!({
            "id": 0x00u8,
            "body": lock.clone()
        });

        drop(lock);
        let mut lock = self.write.lock().await;
        lock.send(Message::Binary(serde_json::to_vec(&body).unwrap())).await.map_err(|e| match e {
            tokio_tungstenite::tungstenite::Error::Io(io) => io,
            x => std::io::Error::new(ErrorKind::Other, x)
        })?;

        Ok(())
    }

    async fn connect_with_token (token: &str) -> std::io::Result<WebSocketStream<MaybeTlsStream<TcpStream>>> {
        let request = Request::builder()
            .uri("ws://127.0.0.1:8080/player/conn")
            .header("Host", "127.0.0.1:8080")
            .header("connection", "Upgrade")
            .header("Upgrade", "websocket")
            .header("Authorization", format!("Bearer {token}").into_bytes())
            .header("sec-websocket-key", base64::encode(random::<[u8;16]>()))
            .header("sec-websocket-version", "13")
            .body(()).unwrap();
        
        let (stream, _) = connect_async(request).await.map_err(|e| match e {
            tokio_tungstenite::tungstenite::Error::Io(io) => io,
            e => std::io::Error::new(ErrorKind::Other, e)
        })?;

        Ok(stream)
    }

    async fn get_player_info (token: &str) -> std::io::Result<PlayerRequest> {
        reqwest::Client::new()
            .get("http://localhost:8080/player")
            .header("Authorization", format!("Bearer {token}"))
            .send().await.map_err(|e| std::io::Error::new(ErrorKind::Other, e))?
            .json().await.map_err(|e| std::io::Error::new(ErrorKind::Other, e))
    }
}