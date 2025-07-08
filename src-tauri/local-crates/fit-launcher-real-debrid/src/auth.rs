use std::sync::Arc;
use tokio::sync::Mutex;
use tauri::{AppHandle, Emitter};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use fit_launcher_config::settings::{
    config::{change_realdebrid_settings, get_realdebrid_settings},
    creation::RealDebridSettings,
};
use crate::{
    client::Client,
    error::RealDebridError,
};

#[derive(Clone)]
pub struct AuthState {
    pub access_token: Arc<Mutex<Option<String>>>,
    pub refresh_token: Arc<Mutex<Option<String>>>,
    pub client_id: Arc<Mutex<String>>,
    pub expires_at: Arc<Mutex<u64>>, // Store as a timestamp
}

impl AuthState {
    pub fn new() -> Self {
        let settings = get_realdebrid_settings();
        Self {
            access_token: Arc::new(Mutex::new(None)),
            refresh_token: Arc::new(Mutex::new(settings.refresh_token)),
            client_id: Arc::new(Mutex::new(settings.client_id)),
            expires_at: Arc::new(Mutex::new(0)),
        }
    }

    pub async fn get_access_token(&self, client: &Client, _app_handle: &AppHandle) -> Result<String, RealDebridError> {
        let mut token = self.access_token.lock().await;
        let expires_at = *self.expires_at.lock().await;

        if token.is_some() && SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() < expires_at {
            return Ok(token.clone().unwrap());
        }

        // Token is expired or not present, try to refresh
        let new_token = self.refresh_access_token(client).await?;
        *token = Some(new_token.clone());
        Ok(new_token)
    }

    async fn refresh_access_token(&self, _client: &Client) -> Result<String, RealDebridError> {
        // This function will need to be implemented, using the refresh token to get a new access token
        // For now, we'll just return an error to indicate it's not implemented
        Err(RealDebridError::Auth("Refresh token flow not implemented.".to_string()))
    }
}

pub async fn initiate_device_auth(
    app_handle: &AppHandle,
    client: &Client,
    auth_state: &AuthState,
) -> Result<(), RealDebridError> {
    let client_id = auth_state.client_id.lock().await.clone();
    let device_code_response = client.get_device_code(&client_id).await?;

    app_handle.emit(
        "realdebrid-auth-prompt",
        device_code_response.verification_url.clone(),
    )?;

    let interval = Duration::from_secs(device_code_response.interval);
    let expires_in = Duration::from_secs(device_code_response.expires_in);
    let start_time = SystemTime::now();

    loop {
        if start_time.elapsed().unwrap() > expires_in {
            return Err(RealDebridError::Auth("Device code expired.".to_string()));
        }

        tokio::time::sleep(interval).await;

        match client.check_device_code(&client_id, &device_code_response.device_code).await {
            Ok(token_response) => {
                let mut access_token = auth_state.access_token.lock().await;
                *access_token = Some(token_response.access_token.clone());

                let mut refresh_token = auth_state.refresh_token.lock().await;
                *refresh_token = Some(token_response.refresh_token.clone());

                let mut expires_at = auth_state.expires_at.lock().await;
                let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                *expires_at = now + token_response.expires_in;
                
                let new_settings = RealDebridSettings {
                    client_id: client_id.clone(),
                    refresh_token: Some(token_response.refresh_token),
                };

                change_realdebrid_settings(new_settings).map_err(|e| RealDebridError::Auth(e.to_string()))?;

                app_handle.emit("realdebrid-auth-success", ())?;
                return Ok(());
            }
            Err(RealDebridError::Api { code, .. }) if code == 8 => { // Authorization pending
                // Continue polling
            }
            Err(e) => {
                app_handle.emit("realdebrid-auth-failure", e.to_string())?;
                return Err(e);
            }
        }
    }
}