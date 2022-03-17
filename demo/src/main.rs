use std::{sync::{Arc}, thread, time::Duration};
use bson::oid::ObjectId;
use local::{PlayerConnection, PlayerLocation};
use llml::vec::{EucVecf2, EucVecd2};
use serde::{Deserialize, Deserializer};
use session::GameSession;
use slg::{renderer::opengl::OpenGl, Renderer, generics::Color};

pub mod local;
pub mod remote;
pub mod session;

fn main() {
    let ogl = Arc::new(OpenGl::new().unwrap());
    let window = ogl.create_instance("Websocket testing", 900u32, 900u32).unwrap();
    let session = GameSession::new(window);

    thread::spawn(move || {
        loop {
            // TODO
            thread::sleep(Duration::from_millis(17));
        }
    });

    ogl.listen_events().unwrap()
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