use reqwest::{header, Client as ReqwestClient, Response, StatusCode};
use serde::de::DeserializeOwned;
use std::collections::HashMap;

use crate::{
    error::RealDebridError,
    model::{
        AddMagnetResponse, DeviceCodeResponse, ErrorResponse, TokenResponse, TorrentInfo,
        UnrestrictLinkResponse,
    },
};

const API_BASE_URL: &str = "https://api.real-debrid.com/rest/1.0";
const OAUTH_BASE_URL: &str = "https://api.real-debrid.com/oauth/v2";

#[derive(Debug, Clone)]
pub struct Client {
    client: ReqwestClient,
}

impl Client {
    pub fn new() -> Self {
        Client {
            client: ReqwestClient::new(),
        }
    }

    async fn handle_response<T: DeserializeOwned>(response: Response) -> Result<T, RealDebridError> {
        match response.status() {
            StatusCode::OK | StatusCode::CREATED | StatusCode::NO_CONTENT => {
                let text = response.text().await?;
                if text.is_empty() {
                    // Handle empty response for 204 No Content
                    serde_json::from_str("{}").map_err(RealDebridError::from)
                } else {
                    serde_json::from_str(&text).map_err(|e| {
                        println!("Failed to deserialize response: {}", text);
                        RealDebridError::SerdeJson(e)
                    })
                }
            }
            _ => {
                let text = response.text().await?;
                let error_response: ErrorResponse = serde_json::from_str(&text)
                    .unwrap_or_else(|_| ErrorResponse {
                        error: format!("Unknown API error: {}", text),
                        error_code: 0,
                    });
                Err(RealDebridError::Api {
                    message: error_response.error,
                    code: error_response.error_code,
                })
            }
        }
    }

    pub async fn get_device_code(&self, client_id: &str) -> Result<DeviceCodeResponse, RealDebridError> {
        let url = format!("{}/device/code?client_id={}", OAUTH_BASE_URL, client_id);
        let response = self.client.get(&url).send().await?;
        Self::handle_response(response).await
    }

    pub async fn check_device_code(
        &self,
        client_id: &str,
        device_code: &str,
    ) -> Result<TokenResponse, RealDebridError> {
        let url = format!("{}/token", OAUTH_BASE_URL);
        let mut params = HashMap::new();
        params.insert("client_id", client_id);
        params.insert("code", device_code);
        params.insert("grant_type", "http://oauth.net/grant_type/device/1.0");

        let response = self.client.post(&url).form(&params).send().await?;
        Self::handle_response(response).await
    }

    pub async fn add_magnet(
        &self,
        magnet_link: &str,
        access_token: &str,
    ) -> Result<AddMagnetResponse, RealDebridError> {
        let url = format!("{}/torrents/addMagnet", API_BASE_URL);
        let mut params = HashMap::new();
        params.insert("magnet", magnet_link);

        let response = self
            .client
            .post(&url)
            .header(header::AUTHORIZATION, format!("Bearer {}", access_token))
            .form(&params)
            .send()
            .await?;
        Self::handle_response(response).await
    }
    
    pub async fn get_torrent_info(
        &self,
        torrent_id: &str,
        access_token: &str,
    ) -> Result<TorrentInfo, RealDebridError> {
        let url = format!("{}/torrents/info/{}", API_BASE_URL, torrent_id);
        let response = self
            .client
            .get(&url)
            .header(header::AUTHORIZATION, format!("Bearer {}", access_token))
            .send()
            .await?;
        Self::handle_response(response).await
    }

    pub async fn select_files(
        &self,
        torrent_id: &str,
        files: &str, // "all" or comma-separated list of file IDs
        access_token: &str,
    ) -> Result<(), RealDebridError> {
        let url = format!("{}/torrents/selectFiles/{}", API_BASE_URL, torrent_id);
        let mut params = HashMap::new();
        params.insert("files", files);

        let response = self
            .client
            .post(&url)
            .header(header::AUTHORIZATION, format!("Bearer {}", access_token))
            .form(&params)
            .send()
            .await?;
        Self::handle_response(response).await
    }

    pub async fn unrestrict_link(
        &self,
        link: &str,
        access_token: &str,
    ) -> Result<UnrestrictLinkResponse, RealDebridError> {
        let url = format!("{}/unrestrict/link", API_BASE_URL);
        let mut params = HashMap::new();
        params.insert("link", link);

        let response = self
            .client
            .post(&url)
            .header(header::AUTHORIZATION, format!("Bearer {}", access_token))
            .form(&params)
            .send()
            .await?;
        Self::handle_response(response).await
    }
}