use std::{net::TcpStream, io::ErrorKind};
use bson::oid::ObjectId;
use llml::vec::{EucVecd2, EucVecf2};
use rand::random;
use serde::{Deserialize, Serialize};
use serde_json::json;
use slg::{generics::{Circle}, renderer::opengl::{OpenGl, GlInstance}, Threadly, RenderInstance};
use websocket::{header::Headers, ClientBuilder, WebSocketResult, WebSocketError, sync::Client, Message};
use reqwest::blocking::get;
use crate::{PlayerRequest, world_to_local, local_to_world};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerLocation {
    pub system: ObjectId,
    pub position: EucVecd2
}

pub struct PlayerConnection {
    pub client: Client<TcpStream>,
    pub location: PlayerLocation,
    pub token: String,
    pub circle: Threadly<Circle<OpenGl>>
}

impl PlayerConnection {
    pub fn new (window: Threadly<GlInstance>) -> std::io::Result<Self> {
        let id = random::<u16>();
        let token = get(format!("http://localhost:8080/test/player/{id}")).map_err(|e| std::io::Error::new(ErrorKind::Other, e))?
            .text().map_err(|e| std::io::Error::new(ErrorKind::Other, e))?;

        let client = Self::connect_with_token(&token)
            .map_err(|e| std::io::Error::new(ErrorKind::Other, e))?;

        let player = Self::get_player_info(&token)?;
        let mut window = window.write().unwrap();

        let circle = window.create_circle(world_to_local(player.location.position), 0.01, player.color)
            .map_err(|e| std::io::Error::new(ErrorKind::Other, e))?;
        
        Ok(Self { client, token, circle, location: player.location })
    }

    pub fn translate (&mut self, delta: EucVecf2) {
        let world = local_to_world(delta);
        self.location.position += world;

        let mut circle = self.circle.write().unwrap();
        circle.position += delta;
        
        let body = json!({
            "id": 0x00u8,
            "body": self.location
        });

        self.client.send_message(&Message::text(serde_json::to_string(&body).unwrap())).unwrap()
    }

    fn connect_with_token (token: &str) -> WebSocketResult<Client<TcpStream>> {
        let mut headers = Headers::new();
        headers.append_raw("Authorization", format!("Bearer {token}").into_bytes());
    
        ClientBuilder::new(&format!("http://127.0.0.1:8080/player/conn"))
            .map_err(|e| WebSocketError::Other(Box::new(e)))?
            .custom_headers(&headers)
            .connect_insecure()
    }

    fn get_player_info (token: &str) -> std::io::Result<PlayerRequest> {
        reqwest::blocking::Client::new()
            .get("http://localhost:8080/player")
            .header("Authorization", format!("Bearer {token}"))
            .send().map_err(|e| std::io::Error::new(ErrorKind::Other, e))?
            .json().map_err(|e| std::io::Error::new(ErrorKind::Other, e))
    }
}