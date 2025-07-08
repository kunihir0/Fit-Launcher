use directories::BaseDirs;
use serde::Serialize;
use std::fmt;
use std::fs;
use tracing::error;
use tracing::info;

use crate::client::dns::FitLauncherDnsConfig;

use super::creation::{GamehubSettings, InstallationSettings, RealDebridSettings};

#[derive(Debug, Serialize)]
pub struct SettingsConfigurationError {
    message: String,
}

impl fmt::Display for SettingsConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for SettingsConfigurationError {}

impl From<reqwest::Error> for SettingsConfigurationError {
    fn from(error: reqwest::Error) -> Self {
        SettingsConfigurationError {
            message: error.to_string(),
        }
    }
}

impl From<std::io::Error> for SettingsConfigurationError {
    fn from(error: std::io::Error) -> Self {
        SettingsConfigurationError {
            message: error.to_string(),
        }
    }
}

impl From<serde_json::Error> for SettingsConfigurationError {
    fn from(error: serde_json::Error) -> Self {
        SettingsConfigurationError {
            message: error.to_string(),
        }
    }
}

#[tauri::command]
pub fn get_installation_settings() -> InstallationSettings {
    let base_dirs = BaseDirs::new()
        .ok_or_else(|| error!("Failed to determine base directories"))
        .unwrap();

    let installation_file_path = base_dirs
        .config_dir()
        .join("com.fitlauncher.carrotrub")
        .join("fitgirlConfig")
        .join("settings")
        .join("installation")
        .join("installation.json");

    let file_content = fs::read_to_string(&installation_file_path)
        .map_err(|err| {
            error!(
                "Error reading the file at {:?}: {:#?}",
                installation_file_path, err
            );
        })
        .unwrap_or("{}".to_string());

    serde_json::from_str::<InstallationSettings>(&file_content).unwrap_or_default()
}

#[tauri::command]
pub fn get_gamehub_settings() -> GamehubSettings {
    let base_dirs = BaseDirs::new()
        .ok_or_else(|| error!("Failed to determine base directories"))
        .unwrap();

    let installation_file_path = base_dirs
        .config_dir()
        .join("com.fitlauncher.carrotrub")
        .join("fitgirlConfig")
        .join("settings")
        .join("gamehub")
        .join("gamehub.json");

    let file_content = fs::read_to_string(&installation_file_path)
        .map_err(|err| {
            error!(
                "Error reading the file at {:?}: {:#?}",
                installation_file_path, err
            );
        })
        .unwrap_or("{}".to_string());

    serde_json::from_str::<GamehubSettings>(&file_content).unwrap_or_default()
}

#[tauri::command]
pub fn get_dns_settings() -> FitLauncherDnsConfig {
    let base_dirs = BaseDirs::new()
        .ok_or_else(|| error!("Failed to determine base directories"))
        .unwrap();

    let installation_file_path = base_dirs
        .config_dir()
        .join("com.fitlauncher.carrotrub")
        .join("fitgirlConfig")
        .join("settings")
        .join("dns")
        .join("dns.json");

    let file_content = fs::read_to_string(&installation_file_path)
        .map_err(|err| {
            error!(
                "Error reading the file at {:?}: {:#?}",
                installation_file_path, err
            );
        })
        .unwrap_or("{}".to_string());

    serde_json::from_str::<FitLauncherDnsConfig>(&file_content).unwrap_or_default()
}

#[tauri::command]
pub fn change_installation_settings(
    settings: InstallationSettings,
) -> Result<(), SettingsConfigurationError> {
    let base_dirs = BaseDirs::new().ok_or_else(|| SettingsConfigurationError {
        message: "Failed to determine base directories".to_string(),
    })?;
    let installation_file_path = base_dirs
        .config_dir()
        .join("com.fitlauncher.carrotrub")
        .join("fitgirlConfig")
        .join("settings")
        .join("installation")
        .join("installation.json");

    let settings_json_string =
        serde_json::to_string_pretty(&settings).map_err(SettingsConfigurationError::from)?;

    fs::write(installation_file_path, settings_json_string)
        .map_err(SettingsConfigurationError::from)?;
    Ok(())
}

#[tauri::command]
pub fn change_gamehub_settings(
    settings: GamehubSettings,
) -> Result<(), SettingsConfigurationError> {
    let base_dirs = BaseDirs::new().ok_or_else(|| SettingsConfigurationError {
        message: "Failed to determine base directories".to_string(),
    })?;
    let installation_file_path = base_dirs
        .config_dir()
        .join("com.fitlauncher.carrotrub")
        .join("fitgirlConfig")
        .join("settings")
        .join("gamehub")
        .join("gamehub.json");

    let settings_json_string =
        serde_json::to_string_pretty(&settings).map_err(SettingsConfigurationError::from)?;

    fs::write(installation_file_path, settings_json_string)
        .map_err(SettingsConfigurationError::from)?;
    Ok(())
}

#[tauri::command]
pub fn change_dns_settings(
    settings: FitLauncherDnsConfig,
) -> Result<(), SettingsConfigurationError> {
    let base_dirs = BaseDirs::new().ok_or_else(|| SettingsConfigurationError {
        message: "Failed to determine base directories".to_string(),
    })?;
    let installation_file_path = base_dirs
        .config_dir()
        .join("com.fitlauncher.carrotrub")
        .join("fitgirlConfig")
        .join("settings")
        .join("dns")
        .join("dns.json");

    let settings_json_string =
        serde_json::to_string_pretty(&settings).map_err(SettingsConfigurationError::from)?;

    fs::write(installation_file_path, settings_json_string)
        .map_err(SettingsConfigurationError::from)?;
    Ok(())
}

#[tauri::command]
pub fn reset_installation_settings() -> Result<(), SettingsConfigurationError> {
    let base_dirs = BaseDirs::new().ok_or_else(|| SettingsConfigurationError {
        message: "Failed to determine base directories".to_string(),
    })?;
    let installation_file_path = base_dirs
        .config_dir()
        .join("com.fitlauncher.carrotrub")
        .join("fitgirlConfig")
        .join("settings")
        .join("installation")
        .join("installation.json");

    let settings_json_string = serde_json::to_string_pretty(&InstallationSettings::default())
        .map_err(SettingsConfigurationError::from)?;

    fs::write(installation_file_path, settings_json_string)
        .map_err(SettingsConfigurationError::from)?;
    Ok(())
}

#[tauri::command]
pub fn reset_gamehub_settings() -> Result<(), SettingsConfigurationError> {
    let base_dirs = BaseDirs::new().ok_or_else(|| SettingsConfigurationError {
        message: "Failed to determine base directories".to_string(),
    })?;
    let gamehub_file_path = base_dirs
        .config_dir()
        .join("com.fitlauncher.carrotrub")
        .join("fitgirlConfig")
        .join("settings")
        .join("gamehub")
        .join("gamehub.json");

    let settings_json_string = serde_json::to_string_pretty(&GamehubSettings::default())
        .map_err(SettingsConfigurationError::from)?;

    fs::write(gamehub_file_path, settings_json_string).map_err(SettingsConfigurationError::from)?;
    Ok(())
}

#[tauri::command]
pub fn reset_dns_settings() -> Result<(), SettingsConfigurationError> {
    let base_dirs = BaseDirs::new().ok_or_else(|| SettingsConfigurationError {
        message: "Failed to determine base directories".to_string(),
    })?;
    let dns_file_path = base_dirs
        .config_dir()
        .join("com.fitlauncher.carrotrub")
        .join("fitgirlConfig")
        .join("settings")
        .join("dns")
        .join("dns.json");

    let settings_json_string = serde_json::to_string_pretty(&FitLauncherDnsConfig::default())
        .map_err(SettingsConfigurationError::from)?;

    fs::write(dns_file_path, settings_json_string).map_err(SettingsConfigurationError::from)?;
    Ok(())
}

#[tauri::command]
pub async fn clear_all_cache() -> Result<(), SettingsConfigurationError> {
    let image_cache_path = directories::BaseDirs::new()
        .expect("Could not determine base directories")
        .config_local_dir() // Points to AppData\Local (or equivalent on other platforms)
        .join("com.fitlauncher.carrotrub")
        .join("image_cache.json");

    tokio::fs::remove_file(image_cache_path).await?;
    Ok(())
}

#[tauri::command]
pub fn open_logs_directory() -> Result<(), String> {
    let path = directories::BaseDirs::new()
        .expect("Could not determine base directories")
        .config_dir()
        .join("com.fitlauncher.carrotrub")
        .join("logs");

    // Detect OS
    if cfg!(target_os = "windows") {
        match std::process::Command::new("explorer").arg(path).spawn() {
            Ok(child) => {
                info!("file explorer started with PID: {}", child.id());
            }
            Err(e) => {
                error!("Failed to start file explorer: {}", e);
            }
        }
    } else if cfg!(target_os = "macos") {
        match std::process::Command::new("open").arg(path).spawn() {
            Ok(child) => {
                info!("file explorer started with PID: {}", child.id());
            }
            Err(e) => {
                error!("Failed to start file explorer: {}", e);
            }
        }
    } else {
        match std::process::Command::new("xdg-open").arg(path).spawn() {
            Ok(child) => {
                info!("file explorer started with PID: {}", child.id());
            }
            Err(e) => {
                error!("Failed to start file explorer: {}", e);
            }
        }
    }

        Ok(())
    }
    
    #[tauri::command]
    pub fn get_realdebrid_settings() -> RealDebridSettings {
        let base_dirs = BaseDirs::new()
            .ok_or_else(|| error!("Failed to determine base directories"))
            .unwrap();
    
        let rd_file_path = base_dirs
            .config_dir()
            .join("com.fitlauncher.carrotrub")
            .join("fitgirlConfig")
            .join("settings")
            .join("realdebrid")
            .join("realdebrid.json");
    
        let file_content = fs::read_to_string(&rd_file_path)
            .map_err(|err| {
                error!(
                    "Error reading the file at {:?}: {:#?}",
                    rd_file_path, err
                );
            })
            .unwrap_or("{}".to_string());
    
        serde_json::from_str::<RealDebridSettings>(&file_content).unwrap_or_default()
    }
    
    #[tauri::command]
    pub fn change_realdebrid_settings(
        settings: RealDebridSettings,
    ) -> Result<(), SettingsConfigurationError> {
        let base_dirs = BaseDirs::new().ok_or_else(|| SettingsConfigurationError {
            message: "Failed to determine base directories".to_string(),
        })?;
        let rd_file_path = base_dirs
            .config_dir()
            .join("com.fitlauncher.carrotrub")
            .join("fitgirlConfig")
            .join("settings")
            .join("realdebrid")
            .join("realdebrid.json");
    
        let settings_json_string =
            serde_json::to_string_pretty(&settings).map_err(SettingsConfigurationError::from)?;
    
        fs::write(rd_file_path, settings_json_string)
            .map_err(SettingsConfigurationError::from)?;
        Ok(())
    }
