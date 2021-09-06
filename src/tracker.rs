use crate::config::TorrustConfig;
use std::sync::Arc;
use crate::database::Database;
use crate::models::tracker_key::TrackerKey;
use crate::errors::ServiceError;
use crate::models::user::User;
use serde::{Serialize, Deserialize};

pub struct TrackerService {
    cfg: Arc<TorrustConfig>,
    database: Arc<Database>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TorrentInfo {
    pub info_hash: String,
    //pub completed: i64,
    pub seeders: i64,
    pub leechers: i64,
    pub peers: Vec<Vec<Peer>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Peer {
    pub id: Option<String>,
    pub client: Option<String>,
    pub ip: Option<String>,
    pub updated: Option<i64>,
    pub uploaded: Option<i64>,
    pub downloaded: Option<i64>,
    pub left: Option<i64>,
    pub event: Option<String>,
}

impl TrackerService {
    pub fn new(cfg: Arc<TorrustConfig>, database: Arc<Database>) -> TrackerService {
        TrackerService {
            cfg,
            database
        }
    }

    pub async fn whitelist_info_hash(&self, info_hash: String) -> Result<(), ServiceError> {
        let request_url =
            format!("{}/api/whitelist/{}?token={}", self.cfg.tracker.api_url, info_hash, self.cfg.tracker.token);

        let client = reqwest::Client::new();

        let response = match client.post(request_url).send().await {
            Ok(v) => Ok(v),
            Err(_) => Err(ServiceError::InternalServerError)
        }?;

        if response.status().is_success() {
            return Ok(())
        }

        Err(ServiceError::InternalServerError)
    }

    pub async fn get_personal_announce_url(&self, user: &User) -> Result<String, ServiceError> {
        let mut tracker_key = self.database.get_valid_tracker_key(user.user_id).await;

        match tracker_key {
            Some(v) => { Ok(format!("{}/{}", self.cfg.tracker.url, v.key)) }
            None => {
                match self.retrieve_new_tracker_key(user.user_id).await {
                    Ok(v) => { Ok(format!("{}/{}", self.cfg.tracker.url, v.key)) },
                    Err(_) => { Err(ServiceError::TrackerOffline) }
                }
            }
        }
    }

    pub async fn retrieve_new_tracker_key(&self, user_id: i64) -> Result<TrackerKey, ServiceError> {
        let request_url =
            format!("{}/api/key/{}?token={}", self.cfg.tracker.api_url, self.cfg.tracker.token_valid_seconds, self.cfg.tracker.token);

        let client = reqwest::Client::new();
        let response = match client.post(request_url)
            .send()
            .await {
            Ok(v) => Ok(v),
            Err(_) => Err(ServiceError::InternalServerError)
        }?;

        let tracker_key: TrackerKey = match response.json::<TrackerKey>().await {
            Ok(v) => Ok(v),
            Err(_) => Err(ServiceError::InternalServerError)
        }?;

        println!("{:?}", tracker_key);

        self.database.issue_tracker_key(&tracker_key, user_id).await?;

        Ok(tracker_key)
    }

    pub async fn get_torrent_info(&self, info_hash: &str) -> Result<TorrentInfo, ServiceError> {
        let request_url =
            format!("{}/api/torrent/{}?token={}", self.cfg.tracker.api_url, info_hash, self.cfg.tracker.token);

        let client = reqwest::Client::new();
        let response = match client.get(request_url)
            .send()
            .await {
            Ok(v) => Ok(v),
            Err(_) => Err(ServiceError::InternalServerError)
        }?;

        let torrent_info = match response.json::<TorrentInfo>().await {
            Ok(v) => Ok(v),
            Err(e) => {
                println!("{:?}", e);
                Err(ServiceError::InternalServerError)
            }
        }?;

        Ok(torrent_info)
    }
}
