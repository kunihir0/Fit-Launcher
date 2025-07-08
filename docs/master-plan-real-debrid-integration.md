# **Master Plan: Real-Debrid Integration and Legacy System Decommissioning**

## 1. Objective

To completely replace the existing multi-backend download system (Aria2, Torrent, DDL) with a unified, singular download backend powered by the Real-Debrid (RD) API. This will simplify the user experience, reduce maintenance overhead, and streamline the application's architecture.

## 2. Phase 1: Decommissioning Legacy Download Systems

This phase focuses on the systematic removal of all components related to the old download infrastructure.

### **2.1. Backend (Rust)**

1.  **Modify `src-tauri/Cargo.toml`:**
    *   Remove the following lines from the `[dependencies]` section:
        *   `fit-launcher-torrent = { path = "./local-crates/fit-launcher-torrent" }`
        *   `fit-launcher-aria2 = { path = "./local-crates/fit-launcher-aria2" }`
        *   `fit-launcher-ddl = { path = "./local-crates/fit-launcher-ddl" }`
    *   Remove the corresponding packages from the `[workspace]` members list.
    *   Remove the following from the `[workspace.dependencies]` section:
        *   `aria2-ws = ...`
        *   `librqbit = ...`
2.  **Refactor `src-tauri/src/main.rs`:**
    *   Remove `use` statements for `fit_launcher_torrent::functions::TorrentSession` and `fit_launcher_torrent::functions::ARIA2_DAEMON`.
    *   Remove the `.manage(TorrentSession::new().await)` line from the Tauri builder.
    *   Remove the graceful shutdown logic for `ARIA2_DAEMON` within the `.on_menu_event` closure for the "quit" event.
3.  **Delete Legacy Crates:**
    *   Delete the following directories entirely:
        *   `src-tauri/local-crates/fit-launcher-aria2/`
        *   `src-tauri/local-crates/fit-launcher-torrent/`
        *   `src-tauri/local-crates/fit-launcher-ddl/`
4.  **Clean `fit-launcher-config`:**
    *   Audit the `src-tauri/local-crates/fit-launcher-config/` crate to remove any structs or functions related to torrent/Aria2 configuration.

### **2.2. Frontend (SolidJS)**

1.  **Delete Components/Pages:**
    *   `src/pages/Settings-01/Settings-Categories/Torrenting/Torrenting.jsx`
    *   `src/pages/Downloads-01/Downloads-Page.jsx`
2.  **Refactor Components/Pages:**
    *   `src/pages/Download-Game-UUID-01/Download-Game-UUID.jsx`: Remove all logic related to fetching magnet links and initiating torrent downloads. This page will be repurposed to trigger the Real-Debrid flow.
    *   `src/Pop-Ups/Download-PopUp/Download-PopUp.jsx`: Gut the existing UI and logic; it will be replaced with a Real-Debrid status monitor.
    *   `src/components/functions/dataStoreGlobal.jsx`: Remove any stores or signals related to torrent clients, download lists, and progress tracking.
    *   `src/pages/Settings-01/Settings.jsx`: Remove the navigation entry for the Torrenting settings page.

## 3. Phase 2: Architecting the `fit-launcher-real-debrid` Crate

A new, self-contained crate will be created at `src-tauri/local-crates/fit-launcher-real-debrid/`.

### **3.1. Crate Structure**

```
fit-launcher-real-debrid/
├── Cargo.toml
└── src/
    ├── lib.rs         # Main library entry point
    ├── commands.rs    # All Tauri commands exposed to the frontend
    ├── model.rs       # Structs for API requests and responses (serde)
    ├── auth.rs        # OAuth2 device flow logic and token management
    ├── client.rs      # The core API client for making requests to RD
    └── error.rs       # Crate-specific error types
```

### **3.2. Token Management (`auth.rs` & `fit-launcher-config`)**

This follows the `plan-real-debrid-token-management.md` document precisely.

1.  **`fit-launcher-config` Crate:**
    *   Will be modified to include a `RealDebridSettings` struct and `create_realdebrid_settings_file`, `get_realdebrid_settings`, `change_realdebrid_settings` commands. This crate acts as the **persistent, secure storage for the `client_id` and long-lived `refresh_token` only.**
2.  **`fit-launcher-real-debrid` Crate:**
    *   Will contain an `ApiState` struct managed by Tauri:
        ```rust
        // in fit-launcher-real-debrid/src/lib.rs
        pub struct ApiState {
            pub client: reqwest::Client,
            pub access_token: Arc<Mutex<Option<String>>>,
            pub token_expiry: Arc<Mutex<Option<i64>>>, // Store expiry as a UTC timestamp
        }
        ```
    *   The `auth.rs` module will be responsible for the full "OAuth2 for Devices" flow.
    *   It will **never** persist the `refresh_token` itself. Instead, it will call the `change_realdebrid_settings` command from the `fit-launcher-config` crate to delegate storage.
    *   It will handle refreshing the `access_token` using the `refresh_token` (which it retrieves on-demand via `get_realdebrid_settings`).

## 4. Phase 3: Core Integration

### **4.1. Backend (Rust)**

1.  **Create the new crate:**
    *   `cargo new --lib src-tauri/local-crates/fit-launcher-real-debrid`
2.  **Update `src-tauri/Cargo.toml`:**
    *   Add `fit-launcher-real-debrid = { path = "./local-crates/fit-launcher-real-debrid" }` to `[dependencies]` and `[workspace.members]`.
3.  **Update `src-tauri/src/main.rs`:**
    *   In the `setup` closure, add `.manage(ApiState::new())` to the Tauri builder.
    *   The existing `tauri_helper::tauri_collect_commands!()` will automatically pick up the new commands from the `fit-launcher-real-debrid` crate.

## 5. Phase 4: Frontend Implementation

1.  **New Settings Component:**
    *   Create a new settings page/component: `src/pages/Settings-01/Settings-Categories/RealDebrid/RealDebrid.jsx`.
    *   This component will have a "Login to Real-Debrid" button.
    *   On click, it will invoke the `rd_authenticate` Tauri command.
    *   It will listen for a `realdebrid-auth-prompt` event from the backend and display the `user_code` and `verification_url` in a modal.
    *   It will listen for a `realdebrid-auth-success` or `realdebrid-auth-failure` event to update the UI state.
2.  **Refactored Download Flow:**
    *   The user action to download a game will now trigger a single Tauri command, e.g., `rd_add_magnet(magnet_url: String)`.
    *   The frontend will no longer be concerned with file selection initially. The backend will handle the `addMagnet` -> `selectFiles` flow.
3.  **New Download Status UI:**
    *   A new global component or popup will be created to display the status of active Real-Debrid torrents.
    *   This component will periodically invoke a `rd_get_torrents` command to refresh the status of all active items. It will display statuses like "Downloading", "Waiting for file selection", "Ready for download", "Error".
    *   When a torrent's status is "Ready", a download button will appear, which triggers the final `rd_unrestrict_and_download(link: String)` command.

## 6. Architectural Diagram

This diagram illustrates the final, unified architecture.

```mermaid
flowchart TD
    subgraph Frontend (SolidJS)
        A[Settings UI] -->|Invoke Command| B(Tauri API)
        C[Download Button] -->|Invoke Command| B
        D[Status UI] -->|Invoke Command & Listen for Events| B
    end

    subgraph Backend (Tauri Core)
        B -- Command --> E{Command Router}
    end

    subgraph "fit-launcher-config (Crate)"
        F["realdebrid.json</br>(client_id, refresh_token)"]
        G[Config Commands</br>get_realdebrid_settings</br>change_realdebrid_settings]
        F <--> G
    end

    subgraph "fit-launcher-real-debrid (Crate)"
        H[RD Commands</br>rd_authenticate</br>rd_add_magnet</br>...]
        I[Auth Module</br>(OAuth2 Flow)]
        J[API Client</br>(reqwest)]
        K[ApiState</br>(in-memory access_token)]
        
        H --> I
        H --> J
        J --> K
        I --> J
    end
    
    subgraph "Real-Debrid API"
        L[OAuth Endpoint]
        M[REST API Endpoint]
    end

    E --"rd_*"--> H
    E --"config_*"--> G
    I --"get/set refresh_token"--> G
    I --"Device Flow"--> L
    J --"API Calls with Bearer token"--> M

    style F fill:#f9f,stroke:#333,stroke-width:2px
    style K fill:#ccf,stroke:#333,stroke-width:2px
```

## 7. Risk Assessment & Mitigation

*   **Risk:** Real-Debrid API rate limits (250 req/min) could be hit by aggressive status polling.
    *   **Mitigation:** Implement intelligent polling with exponential backoff. The status polling interval on the frontend should be configurable and default to a reasonable value (e.g., 5-10 seconds).
*   **Risk:** Unhandled Real-Debrid API errors could lead to a confusing user experience.
    *   **Mitigation:** The `fit-launcher-real-debrid` crate's `error.rs` will map every known RD error code to a specific, typed error. The backend commands will translate these into user-friendly messages for the frontend.
*   **Risk:** The OAuth device flow can time out, or the user can abandon it.
    *   **Mitigation:** The `rd_authenticate` command will have a timeout. The frontend modal displaying the `user_code` will have a countdown timer and a "Cancel" button.
*   **Risk:** A change in the Real-Debrid API could break the integration.
    *   **Mitigation:** The `fit-launcher-real-debrid` crate isolates all API logic. Future changes will only require updating this single, modular crate. Add integration tests that mock the RD API to catch regressions.