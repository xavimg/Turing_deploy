use std::{collections::HashMap, io::ErrorKind};
use bson::oid::ObjectId;
use slg::{Threadly, renderer::opengl::{GlInstance, OpenGl}, generics::Circle};
use crate::{remote::RemotePlayer, local::PlayerConnection, PlayerRequest};

pub struct GameSession {
    pub window: Threadly<GlInstance>,
    pub local: PlayerConnection,
    pub remote: HashMap<ObjectId, RemotePlayer>
}

impl GameSession {
    pub fn new (window: Threadly<GlInstance>) -> std::io::Result<Self> {
        let local = PlayerConnection::new(window.clone())?;
        let others : Vec<PlayerRequest> = reqwest::blocking::Client::new()
            .get("http://localhost:8080/system/players")
            .header("Authorization", format!("Bearer {}", local.token))
            .send().map_err(|e| std::io::Error::new(ErrorKind::Other, e))?
            .json().map_err(|e| std::io::Error::new(ErrorKind::Other, e))?;

        let mut remote = HashMap::with_capacity(others.len());
        for player in others {
            remote.insert(player.id, RemotePlayer::new(player.location, player.color, window.clone()));
        }

        Ok(Self { window, local, remote })
    }

    #[inline]
    pub fn circles (&self) -> impl Iterator<Item = &Threadly<Circle<OpenGl>>> {
        let local = std::iter::once(&self.local.circle);
        let remote = self.remote.values().map(|x| &x.circle);
        local.chain(remote)
    }

    pub fn listen (self) {
        loop {
            
        }
    }
}