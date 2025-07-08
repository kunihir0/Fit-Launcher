use tauri::{command, AppHandle, State};
use crate::{
    auth::{initiate_device_auth, AuthState},
    client::Client,
    error::RealDebridError,
};

#[command]
pub async fn rd_authenticate(
    app_handle: AppHandle,
    auth_state: State<'_, AuthState>,
    client: State<'_, Client>,
) -> Result<(), RealDebridError> {
    let auth_state = auth_state.inner().clone();
    let client = client.inner().clone();
    tokio::spawn(async move {
        if let Err(e) = initiate_device_auth(&app_handle, &client, &auth_state).await {
            eprintln!("Authentication failed: {}", e);
        }
    });
    Ok(())
}

#[command]
pub async fn rd_add_magnet(
    app_handle: AppHandle,
    auth_state: State<'_, AuthState>,
    client: State<'_, Client>,
    magnet: String,
) -> Result<(), RealDebridError> {
    let access_token = auth_state.get_access_token(&client, &app_handle).await?;
    let add_magnet_response = client.add_magnet(&magnet, &access_token).await?;
    
    // After adding the magnet, we must select the files to start the download.
    // For simplicity, we will select all files.
    client.select_files(&add_magnet_response.id, "all", &access_token).await?;

    // Optionally, you can start polling for the torrent status here
    // and emit events to the frontend.
    
    Ok(())
}