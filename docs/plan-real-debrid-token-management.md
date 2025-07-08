# Plan: Real-Debrid Token Storage and Management

This plan outlines the strategy for securely storing and managing Real-Debrid authentication tokens within Fit-Launcher, emphasizing a minimal and modular approach for the `fit-launcher-real-debrid` crate.

The core idea is to leverage the existing `fit-launcher-config` crate in the Rust backend for persistent storage of the long-lived `refresh_token` and `client_id`, while the `fit-launcher-real-debrid` crate will handle the `access_token` lifecycle (obtaining, refreshing, and in-memory storage). The frontend will interact with the `fit-launcher-real-debrid` crate via Tauri commands and manage the short-lived `access_token` for immediate API calls.

## 1. Backend (Rust - `fit-launcher-config` crate)

**Goal:** Securely store and retrieve `client_id` and `refresh_token` persistently.

This crate is responsible for managing application-wide configuration, making it the ideal place for persistent storage of sensitive, long-lived tokens like the Real-Debrid `refresh_token` and the application's `client_id`.

*   **File: `src-tauri/local-crates/fit-launcher-config/src/settings/creation.rs`**
    *   **New Struct: `RealDebridSettings`**
        A new Rust struct will be defined to hold the Real-Debrid specific configuration.
        ```rust
        #[derive(Debug, serde::Serialize, serde::Deserialize)]
        pub struct RealDebridSettings {
            pub client_id: String,
            pub refresh_token: Option<String>, // Option because it might not exist initially
        }

        impl Default for RealDebridSettings {
            fn default() -> Self {
                // Using the generic open-source client_id for initial setup.
                // This can be replaced with a dedicated client_id if registered.
                RealDebridSettings {
                    client_id: "X245A4XAIBGVM".to_string(),
                    refresh_token: None,
                }
            }
        }
        ```
    *   **New Function: `create_realdebrid_settings_file()`**
        This function will ensure that a `realdebrid.json` configuration file is created with default settings on application startup if it doesn't already exist. It will follow the existing pattern of other `create_*_settings_file` functions in this module.
        ```rust
        // Example (conceptual):
        pub fn create_realdebrid_settings_file() -> Result<(), std::io::Error> {
            // Logic to create directory:
            // config_dir()/com.fitlauncher.carrotrub/fitgirlConfig/settings/realdebrid/
            // Logic to create file: realdebrid.json with RealDebridSettings::default()
        }
        ```

*   **File: `src-tauri/local-crates/fit-launcher-config/src/settings/config.rs`**
    *   **New Tauri Commands:**
        *   **`get_realdebrid_settings()`**: A Tauri command to read the `RealDebridSettings` from `realdebrid.json`. This command will be accessible by other Rust modules (e.g., `fit-launcher-real-debrid`) and potentially the frontend for initial setup checks.
            ```rust
            #[tauri::command]
            pub fn get_realdebrid_settings() -> RealDebridSettings {
                // Logic to read and deserialize realdebrid.json
            }
            ```
        *   **`change_realdebrid_settings(settings: RealDebridSettings)`**: A Tauri command to update and persist the `RealDebridSettings` to `realdebrid.json`. This will be crucial for saving the `refresh_token` after a successful authentication or refresh.
            ```rust
            #[tauri::command]
            pub fn change_realdebrid_settings(
                settings: RealDebridSettings,
            ) -> Result<(), SettingsConfigurationError> {
                // Logic to serialize and write to realdebrid.json
            }
            ```

## 2. Backend (Rust - `fit-launcher-real-debrid` crate)

**Goal:** Manage the `access_token` lifecycle and facilitate Real-Debrid API interactions. This crate will be minimal and modular, focusing solely on Real-Debrid specific logic.

*   **Token Management Logic:**
    *   **Initialization:** On startup or when the Real-Debrid service is first accessed, the `fit-launcher-real-debrid` crate will use `get_realdebrid_settings()` to load the `client_id` and any existing `refresh_token`.
    *   **OAuth2 Device Flow Implementation:** It will implement the full OAuth2 device flow (as detailed in `docs/real-debrid-api.md`) to obtain an `access_token`.
        *   If a `refresh_token` is available, it will attempt to use it to get a new `access_token` first.
        *   If no `refresh_token` is available, or if refreshing fails, it will initiate the device code flow, prompting the user for interaction via the frontend.
    *   **Access Token Storage (In-Memory):** The short-lived `access_token` will be stored in memory within this crate (e.g., using a `std::sync::Mutex<Option<String>>` or similar mechanism) for efficient access during subsequent API calls.
    *   **Token Refresh Logic:** Proactive refreshing of the `access_token` will be implemented using the `refresh_token` before the `access_token` expires. This will happen in a background task or as part of an interceptor for API calls.
    *   **Refresh Token Persistence:** If a new `refresh_token` is issued during the refresh process, it will be saved persistently by calling `change_realdebrid_settings()` in the `fit-launcher-config` crate.

*   **Tauri Commands (in `fit-launcher-real-debrid/src/commands.rs`):**
    *   **`authenticate_real_debrid()`**: This command will initiate the authentication flow. It will return the `user_code` and `verification_url` to the frontend, and then handle the polling for the `access_token` in the background. Once successful, it will emit a Tauri event to the frontend.
    *   **`get_real_debrid_access_token()`**: A command to retrieve the current valid `access_token` from the in-memory storage of the `fit-launcher-real-debrid` crate. This token will then be used by the frontend when constructing API calls.

## 3. Frontend (SolidJS - `src/components/functions/dataStoreGlobal.jsx` and other UI components)

**Goal:** Trigger authentication, display user prompts, and manage the session-based `access_token`.

*   **No Direct `refresh_token` Storage:** The frontend will not directly store the `refresh_token`. Its management is solely the responsibility of the Rust backend.
*   **Authentication Trigger:** A UI element (e.g., in settings) will trigger the `authenticate_real_debrid()` Tauri command.
*   **User Interaction:** The frontend will display the `user_code` and `verification_url` received from the backend during the device code flow, guiding the user through the Real-Debrid website verification.
*   **Access Token Management (Session-based):**
    *   When the backend successfully obtains an `access_token` (and the `expires_in` duration), it will emit a Tauri event (e.g., `realdebrid-auth-success`) to the frontend.
    *   The frontend can store this short-lived `access_token` and its expiration timestamp in a `makePersisted` store within `dataStoreGlobal.jsx` or a similar local state management for the current user session. This is primarily for immediate use in frontend-initiated requests, or to display authentication status.
    *   Before making any Real-Debrid related API calls (via backend Tauri commands), the frontend will check if it has a valid, non-expired `access_token`. If not, it will initiate the authentication flow again.
*   **API Call Integration:** Frontend components will pass the `access_token` (obtained from its session store) as a parameter to the relevant backend Tauri commands (e.g., `add_magnet_link(magnet: String, access_token: String)`).

This separation of concerns ensures that the sensitive `refresh_token` is managed and persisted securely on the backend, while the frontend handles the user-facing authentication steps and utilizes the short-lived `access_token` for active API interactions.

```mermaid
sequenceDiagram
    participant FE as Frontend (SolidJS)
    participant Config_Crate as `fit-launcher-config` (Rust)
    participant RD_Crate as `fit-launcher-real-debrid` (Rust)
    participant RealDebrid as Real-Debrid API

    %% Initial Setup / Startup
    FE->>Config_Crate: App Startup / Init
    Config_Crate->>Config_Crate: `create_realdebrid_settings_file()` (ensures `realdebrid.json`)

    %% Authentication Flow (User Initiates)
    FE->>RD_Crate: User clicks "Login to Real-Debrid" (via Tauri command `authenticate_real_debrid()`)
    RD_Crate->>Config_Crate: `get_realdebrid_settings()`
    Config_Crate-->>RD_Crate: Returns `client_id`, `refresh_token` (if any)

    alt `refresh_token` exists and is valid
        RD_Crate->>RealDebrid: POST /oauth/v2/token (with refresh_token)
        RealDebrid-->>RD_Crate: New `access_token`, `expires_in`, new `refresh_token` (optional)
        RD_Crate->>Config_Crate: `change_realdebrid_settings()` (if refresh_token updated)
        Config_Crate-->>RD_Crate: Confirmation
    else `refresh_token` invalid or not present (Device Code Flow)
        RD_Crate->>RealDebrid: GET /oauth/v2/device/code (with `client_id`)
        RealDebrid-->>RD_Crate: `device_code`, `user_code`, `verification_url`, `interval`, `expires_in`
        RD_Crate->>FE: Emit Tauri event "realdebrid-auth-prompt" (with `user_code`, `verification_url`)
        Note over FE,RealDebrid: User visits `verification_url` and enters `user_code`
        loop Polling for access token
            RD_Crate->>RealDebrid: POST /oauth/v2/token (with `device_code`)
            RealDebrid-->>RD_Crate: Pending / Error OR `access_token`, `refresh_token`
            alt Authorized
                break
            end
        end
        RD_Crate->>Config_Crate: `change_realdebrid_settings()` (save new `refresh_token`)
        Config_Crate-->>RD_Crate: Confirmation
    end

    RD_Crate->>FE: Emit Tauri event "realdebrid-auth-success" (with `access_token`, `expires_in`)
    FE->>FE: Store `access_token` in session (e.g., `dataStoreGlobal.jsx`)

    %% Subsequent API Calls
    FE->>RD_Crate: User requests download (via Tauri command, e.g., `add_magnet_link(magnet, access_token)`)
    RD_Crate->>RealDebrid: API Request (e.g., POST /torrents/addMagnet, with Bearer `access_token`)
    RealDebrid-->>RD_Crate: API Response
    RD_Crate-->>FE: Result of API call