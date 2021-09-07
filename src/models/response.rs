use serde::{Deserialize, Serialize};
use std::any::Any;
use crate::models::user::User;
use crate::models::torrent::TorrentListing;
use crate::models::torrent_file::File;

pub enum OkResponses {
    TokenResponse(TokenResponse)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OkResponse<T> {
    pub data: T
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse<T> {
    pub errors: Vec<T>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenResponse {
    pub token: String,
    pub username: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewTorrentResponse {
    pub torrent_id: i64,
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct CategoryResponse {
    pub name: String,
    pub num_torrents: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct TorrentResponse {
    pub torrent_id: i64,
    pub uploader: String,
    pub info_hash: String,
    pub title: String,
    pub description: Option<String>,
    pub category_id: i64,
    pub upload_date: i64,
    pub file_size: i64,
    pub seeders: i64,
    pub leechers: i64,
    pub files: Option<Vec<File>>,
}

impl TorrentResponse {
    pub fn from_listing(torrent_listing: TorrentListing) -> TorrentResponse {
        TorrentResponse {
            torrent_id: torrent_listing.torrent_id,
            uploader: torrent_listing.uploader,
            info_hash: torrent_listing.info_hash,
            title: torrent_listing.title,
            description: torrent_listing.description,
            category_id: torrent_listing.category_id,
            upload_date: torrent_listing.upload_date,
            file_size: torrent_listing.file_size,
            seeders: torrent_listing.seeders,
            leechers: torrent_listing.leechers,
            files: None
        }
    }
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct TorrentsResponse {
    pub total: i32,
    pub results: Vec<TorrentListing>,
}
