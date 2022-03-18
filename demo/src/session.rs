use std::{collections::HashMap, io::ErrorKind};
use bson::oid::ObjectId;
use slg::{Threadly, renderer::opengl::{GlInstance, OpenGl}, generics::Circle};
use tokio::sync::Mutex;
use crate::{remote::RemotePlayer, local::PlayerConnection, PlayerRequest};

pub struct GameSession {
    pub window: Threadly<GlInstance>,
    pub local: Mutex<PlayerConnection>,
    pub remote: HashMap<ObjectId, RemotePlayer>
}

impl GameSession {
    pub async fn new (window: Threadly<GlInstance>) -> std::io::Result<Self> {
        let local = PlayerConnection::new(window.clone()).await?;
        let others : Vec<PlayerRequest> = reqwest::blocking::Client::new()
            .get("http://localhost:8080/system/players")
            .header("Authorization", format!("Bearer {}", local.token))
            .send().map_err(|e| std::io::Error::new(ErrorKind::Other, e))?
            .json().map_err(|e| std::io::Error::new(ErrorKind::Other, e))?;

        let mut remote = HashMap::with_capacity(others.len());
        for player in others {
            remote.insert(player.id, RemotePlayer::new(player.location, player.color, window.clone()));
        }

        let local = Mutex::new(local);
        Ok(Self { window, local, remote })
    }

    #[inline]
    pub async fn circles (&self) -> impl Iterator<Item = Threadly<Circle<OpenGl>>> {
        let lock = self.local.lock().await;
        let local = std::iter::once(lock.circle.clone());
        let remote = self.remote.values().map(|x| x.circle.clone());
        local.chain(remote)
    }

    pub fn listen (self) {
        loop {
            
        }
    }
}