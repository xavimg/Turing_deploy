use std::{net::TcpStream, io::ErrorKind};
use llml::vec::{EucVecd2, EucVecf2};
use rand::random;
use serde::{Deserialize, Deserializer};
use slg::{generics::{Circle, Color}, renderer::opengl::{OpenGl, GlInstance}, Threadly, RenderInstance};
use websocket::{header::Headers, ClientBuilder, WebSocketResult, WebSocketError, sync::Client};
use reqwest::blocking::get;

#[derive(Debug, Deserialize)]
pub struct PlayerLocation {
    system: String,
    #[serde(alias = "pos")]
    position: EucVecd2
}

#[derive(Debug, Deserialize)]
struct PlayerRequest {
    _id: String,
    name: String,
    location: PlayerLocation,
    #[serde(deserialize_with = "deserialize_color")]
    color: Color
}

pub struct PlayerConnection {
    client: Client<TcpStream>,
    location: PlayerLocation,
    token: String,
    circle: Threadly<Circle<OpenGl>>
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

        let circle = window.create_circle(Self::world_to_local(player.location.position), 0.01, player.color)
            .map_err(|e| std::io::Error::new(ErrorKind::Other, e))?;

        Ok(Self { client, token, circle, location: player.location })
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

    #[inline]
    fn world_to_local (pos: EucVecd2) -> EucVecf2 {
        let (x, y) = pos.unzip();
        EucVecf2::new([x as f32, y as f32])
    }
}

#[inline]
fn deserialize_color<'de, D> (deserializer: D) -> Result<Color, D::Error> where D: Deserializer<'de> {
    let value = u32::deserialize(deserializer)?;
    let b = (value & 0xff) as u8;
    let g = ((value >> 8) & 0xff) as u8;
    let r = ((value >> 16) & 0xff) as u8;
    Ok(Color::new(r, g, b))
}