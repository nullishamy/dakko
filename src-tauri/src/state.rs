use std::{
    net::SocketAddr,
    path::PathBuf,
    time::{Duration, Instant},
};

use megalodon::{oauth::TokenData, Megalodon, SNS};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientState {
    pub client_id: String,
    pub client_secret: String,
    pub base_url: String,
    pub instance_type: SNS
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthState {
    pub token: TokenData,
}
pub struct AppState {
    pub client_state: RwLock<Option<ClientState>>,
    pub auth_state: RwLock<Option<AuthState>>,
    pub client: RwLock<Option<Box<dyn Megalodon + Sync + Send>>>,

    pub redirect_addr: SocketAddr,
    pub config_dir: RwLock<Option<PathBuf>>,
}

impl AppState {
    pub fn has_logged_in(&self) -> bool {
        let auth_lock = self
            .auth_state
            .try_read_until(Instant::now().checked_add(Duration::from_secs(10)).unwrap());
        let client_lock = self
            .client
            .try_read_until(Instant::now().checked_add(Duration::from_secs(10)).unwrap());

        if auth_lock.is_none() {
            panic!("could not acquire auth lock in time")
        }

        if client_lock.is_none() {
            panic!("could not acquire client lock in time")
        }

        auth_lock.unwrap().is_some() && client_lock.unwrap().is_some()
    }
}
