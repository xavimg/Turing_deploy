use std::{sync::{Arc}, thread, time::Duration};
use bson::oid::ObjectId;
use futures::{StreamExt, join};
use local::{PlayerLocation};
use llml::vec::{EucVecf2, EucVecd2};
use serde::{Deserialize, Deserializer};
use session::GameSession;
use slg::{renderer::opengl::OpenGl, Renderer, generics::{Color, KeyboardKey}, RenderInstance};

pub mod local;
pub mod remote;
pub mod session;

#[tokio::main]
async fn main() {
    let ogl = Arc::new(OpenGl::new().unwrap());
    let window = ogl.create_instance("Websocket testing", 900u32, 900u32).unwrap();

    let session = Arc::new(GameSession::new(window).await.unwrap());
    let player_session = session.clone();

    // WebSocket Updates
    tokio::spawn(async move {
        loop {
            let mut lock = session.local.client.lock().await;
            if let Some(Ok(msg)) = lock.next().await {
                println!("{}", msg.to_text().unwrap())
            }

            thread::sleep(Duration::from_millis(17))
        }
    });

    // Player updates
    let updates = async move {
        loop {
            let window = player_session.window.read().unwrap();
            let x = if window.is_pressed(KeyboardKey::D) {
                1.0
            } else if window.is_pressed(KeyboardKey::A) {
                -1.0
            } else {
                0.0
            };

            let y = if window.is_pressed(KeyboardKey::W) {
                1.0
            } else if window.is_pressed(KeyboardKey::S) {
                -1.0
            } else {
                0.0
            };

            drop(window);
            if x != 0.0 || y != 0.0 {
                let vel = 0.017 * EucVecf2::new([x, y]).unit();
                player_session.local.translate(vel).await;
            }

            thread::sleep(Duration::from_millis(17))
        }
    };

    let listen = async move {
        ogl.listen_events().unwrap();
    };

    join!(updates, listen);
}

#[derive(Debug, Deserialize)]
pub struct PlayerRequest {
    #[serde(rename = "_id")]
    id: ObjectId,
    name: String,
    location: PlayerLocation,
    #[serde(deserialize_with = "deserialize_color")]
    color: Color
}

#[inline]
fn deserialize_color<'de, D> (deserializer: D) -> Result<Color, D::Error> where D: Deserializer<'de> {
    let value = u32::deserialize(deserializer)?;
    let b = (value & 0xff) as u8;
    let g = ((value >> 8) & 0xff) as u8;
    let r = ((value >> 16) & 0xff) as u8;
    Ok(Color::new(r, g, b))
}

#[inline]
fn world_to_local (pos: EucVecd2) -> EucVecf2 {
    let (x, y) = pos.unzip();
    EucVecf2::new([x as f32, y as f32])
}

#[inline]
fn local_to_world (pos: EucVecf2) -> EucVecd2 {
    let (x, y) = pos.unzip();
    EucVecd2::new([x as f64, y as f64])
}