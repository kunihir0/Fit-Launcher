use serde::{Deserialize, Serialize};

// --- Authentication Models ---

#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceCodeResponse {
    pub device_code: String,
    pub user_code: String,
    pub interval: u64,
    pub expires_in: u64,
    pub verification_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenResponse {
    pub access_token: String,
    pub expires_in: u64,
    pub token_type: String,
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub error: String,
    pub error_code: i64,
}

// --- Torrent Management Models ---

#[derive(Serialize, Deserialize, Debug)]
pub struct AddMagnetResponse {
    pub id: String,
    pub uri: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TorrentInfo {
    pub id: String,
    pub filename: String,
    pub hash: String,
    pub bytes: u64,
    pub host: String,
    pub split: u64,
    pub progress: f64,
    pub status: String,
    pub added: String,
    pub files: Vec<TorrentFile>,
    pub links: Vec<String>,
    #[serde(default)]
    pub ended: Option<String>,
    #[serde(default)]
    pub speed: Option<u64>,
    #[serde(default)]
    pub seeders: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TorrentFile {
    pub id: u64,
    pub path: String,
    pub bytes: u64,
    pub selected: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnrestrictLinkResponse {
    pub id: String,
    pub filename: String,
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    pub filesize: u64,
    pub link: String,
    pub host: String,
    pub chunks: u32,
    pub crc: u32,
    pub download: String,
    pub streamable: u32,
}