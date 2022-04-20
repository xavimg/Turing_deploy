use std::{collections::HashMap, io::ErrorKind, sync::Arc};
use bson::oid::ObjectId;
use slg::{Threadly, renderer::opengl::{GlInstance}};
use tokio::sync::{RwLock};
use crate::{remote::RemotePlayer, local::PlayerConnection, PlayerRequest};

pub struct GameSession {
    pub window: Threadly<GlInstance>,
    pub local: PlayerConnection,
    pub remote: Arc<RwLock<HashMap<ObjectId, RemotePlayer>>>
}

impl GameSession {
    pub async fn new (window: Threadly<GlInstance>) -> std::io::Result<Self> {
        let local = PlayerConnection::new(window.clone()).await?;
        let others : Vec<PlayerRequest> = reqwest::Client::new()
            .get("http://localhost:8080/system/players")
            .header("Authorization", format!("Bearer {}", local.token))
            .send().await.map_err(|e| std::io::Error::new(ErrorKind::Other, e))?
            .json().await.map_err(|e| std::io::Error::new(ErrorKind::Other, e))?;

        let mut remote = HashMap::with_capacity(others.len());
        for player in others {
            remote.insert(player.id, RemotePlayer::new(player.location, player.color, window.clone()));
        }

        let remote = Arc::new(RwLock::new(remote));
        Ok(Self { window, local, remote })
    }
}