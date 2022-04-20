use std::{sync::{Arc}, time::Duration, pin::Pin, io::ErrorKind, str::FromStr};
use bson::oid::ObjectId;
use futures::{StreamExt, Future, SinkExt};
use local::{PlayerLocation};
use llml::vec::{EucVecf2, EucVecd2};
use serde::{Deserialize, Deserializer};
use serde_json::{json, Value};
use session::GameSession;
use slg::{renderer::opengl::OpenGl, Renderer, generics::{Color, KeyboardKey}, RenderInstance};
use tokio_tungstenite::tungstenite::Message;

pub mod local;
pub mod remote;
pub mod session;
pub mod timeout;

#[tokio::main]
async fn main() {
    let ogl = Arc::new(OpenGl::new().unwrap());
    let window = ogl.create_instance("Websocket testing", 900u32, 900u32).unwrap();

    let session = Arc::new(GameSession::new(window).await.unwrap());
    let player_session = session.clone();

    // WebSocket Updates
    tokio::spawn(async move {
        loop {
            let mut lock = session.local.read.lock().await;
            if let Some(Ok(msg)) = lock.next().await {
                let json = serde_json::from_slice::<Value>(&msg.into_data()).unwrap();
                match json["id"].as_u64().unwrap() {
                    // Player movement
                    0x10 => {
                        let body = &json["body"];
                        let oid = ObjectId::from_str(body["player"]["$oid"].as_str().unwrap()).unwrap();

                        let position = &body["position"];
                        let position = EucVecd2::new([position["x"].as_f64().unwrap(), position["y"].as_f64().unwrap()]);

                        let lock = session.remote.read().await;
                        let mut circle = lock[&oid].circle.write().unwrap();
                        circle.position = world_to_local(position);
                    },

                    // New player
                    0x11 => {
                        let body = &json["body"];

                        let mut lock = session.remote.write().await;
                        let oid = ObjectId::from_str(body["_id"]["$oid"].as_str().unwrap()).unwrap();

                        let location = &body["location"];
                        let position = &location["position"];

                        let position = EucVecd2::new([position["x"].as_f64().unwrap(), position["y"].as_f64().unwrap()]);
                        let location = PlayerLocation { 
                            system:  ObjectId::from_str(location["system"]["$oid"].as_str().unwrap()).unwrap(),
                            position
                        };
                        
                        let mut window = session.window.write().unwrap();
                        println!("{:?}", window);
                        let circle = window.create_circle(world_to_local(location.position), 0.01, Color::new(128, 4, 33)).unwrap();

                        println!("hi1");
                        lock.insert(oid, remote::RemotePlayer { location, circle });
                    },

                    _ => todo!()
                }
            }
        }
    });

    // Player updates
    fn updates (session: Arc<GameSession>) -> Pin<Box<dyn Send + Future<Output = ()>>> {
        let lock = session.clone();
        let lock = lock.window.read().unwrap();

        let x = if lock.is_pressed(KeyboardKey::D) {
            1.0
        } else if lock.is_pressed(KeyboardKey::A) {
            -1.0
        } else {
            0.0
        };

        let y = if lock.is_pressed(KeyboardKey::W) {
            1.0
        } else if lock.is_pressed(KeyboardKey::S) {
            -1.0
        } else {
            0.0
        };

        drop(lock);
        Box::pin(async move {
            if x != 0.0 || y != 0.0 {
                let delta = 0.017 * EucVecf2::new([x, y]).unit();
                let world = local_to_world(delta);

                let mut lock = session.local.location.lock().await;
                lock.position += world;

                let mut circle = session.local.circle.write().unwrap();
                circle.position += delta;
                
                let body = json!({
                    "id": 0x00u8,
                    "body": lock.clone()
                });

                drop(lock);
                let clone = session.clone();
                tokio::spawn(async move {
                    let mut lock = clone.local.write.lock().await;
                    lock.send(Message::Binary(serde_json::to_vec(&body).unwrap())).await.map_err(|e| match e {
                        tokio_tungstenite::tungstenite::Error::Io(io) => io,
                        x => std::io::Error::new(ErrorKind::Other, x)
                    }).unwrap();
                });

                //let write = lock.client.get_mut().write(&serde_json::to_vec(&body).unwrap());
                //write.await.unwrap();
                //lock.translate(vel).await;
            }
    
            tokio::time::sleep(Duration::from_millis(17)).await;
            tokio::spawn(updates(session));
        })
    }

    tokio::spawn(updates(player_session));
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