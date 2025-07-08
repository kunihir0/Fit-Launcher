# Real-Debrid API Integration for Fit-Launcher

This document outlines the Real-Debrid API endpoints and authentication methods relevant for Fit-Launcher's integration.

## Base URLs

*   **REST API:** `https://api.real-debrid.com/rest/1.0/`
*   **OAuth2 API:** `https://api.real-debrid.com/oauth/v2/`

## Authentication (OAuth2 for Devices)

Fit-Launcher will primarily use the OAuth2 for Devices workflow, as it's suitable for desktop applications. This method avoids the need to embed sensitive `client_secret` within the client application.

### Workflow Overview

1.  **Get Device Code:** The application requests a `device_code` and `user_code` from the `/device/code` endpoint. The `user_code` is a short, human-readable code the user will enter.
2.  **User Verification:** The user is prompted to visit the `verification_url` (e.g., `real-debrid.com/device`) and enter the `user_code` displayed by Fit-Launcher.
3.  **Poll for Token:** The application repeatedly polls the `/token` endpoint using the obtained `device_code` at a specified `interval` until the user successfully authorizes the application.
4.  **Receive Tokens:** Upon successful authorization, the polling request will return an `access_token` and a `refresh_token`.
5.  **Token Refresh:** The `refresh_token` is a long-lived token used to obtain new `access_token`s after they expire, without requiring the user to re-authenticate.

### Relevant Endpoints for Authentication

*   **`GET /oauth/v2/device/code`**: Obtain device and user codes for the authentication flow.
    *   **Parameters:**
        *   `client_id` (string, required): Your application's client ID. For open-source applications, `X245A4XAIBGVM` can be used.
    *   **Response (JSON):**
        *   `device_code` (string): The device verification code.
        *   `user_code` (string): The user-facing code to enter on the verification website.
        *   `interval` (integer): The minimum interval (in seconds) at which the `/token` endpoint should be polled.
        *   `expires_in` (integer): The lifetime (in seconds) of the `device_code`.
        *   `verification_url` (string): The URL the user should visit to enter the `user_code`.
*   **`POST /oauth/v2/token`**: Poll for the access token or refresh an expired one.
    *   **Parameters (for initial token retrieval after device code):**
        *   `client_id` (string, required): Your application's client ID.
        *   `code` (string, required): The `device_code` obtained from `/device/code`.
        *   `grant_type` (string, required): Must be set to `"http://oauth.net/grant_type/device/1.0"`.
    *   **Parameters (for refreshing an access token):**
        *   `client_id` (string, required): Your application's client ID.
        *   `code` (string, required): The `refresh_token` obtained previously.
        *   `grant_type` (string, required): Must be set to `"http://oauth.net/grant_type/device/1.0"`.
    *   **Response (JSON):**
        *   `access_token` (string): The token to use for API requests.
        *   `expires_in` (integer): Lifetime of the `access_token` in seconds.
        *   `token_type` (string): Always `"Bearer"`.
        *   `refresh_token` (string): Token to obtain new `access_token`s.

### Authentication Considerations

*   **Client ID:** While a generic open-source `client_id` is available, Fit-Launcher should consider registering its own application with Real-Debrid to obtain a dedicated `client_id` for better control and potential higher rate limits.
*   **Token Storage:** The `access_token` and especially the `refresh_token` must be stored securely. Tauri's `tauri-plugin-store` or encrypted local storage should be utilized for this purpose.

## Core API Endpoints for Downloads

All requests to these endpoints require an `Authorization: Bearer <access_token>` header.

### Torrents

These endpoints handle the process of adding and managing torrents on Real-Debrid's servers.

*   **`GET /torrents`**: Retrieve a list of all torrents associated with the user's Real-Debrid account.
*   **`GET /torrents/info/{id}`**: Get detailed information about a specific torrent by its ID. This endpoint is crucial for checking the status of a torrent and retrieving the direct download links once it's processed.
*   **`POST /torrents/addMagnet`**: Add a torrent to Real-Debrid's servers using a magnet link.
    *   **Parameters:**
        *   `magnet` (string, required): The magnet URI of the torrent.
    *   **Response (JSON):** Contains details about the added torrent, including its `id`, `name`, `hash`, `bytes`, and a list of `files` with their respective `id`, `path`, `bytes`, and `selected` status.
*   **`POST /torrents/selectFiles/{id}`**: Select which files within a torrent should be made available for download. After adding a magnet, this step is necessary to initiate the unrestrict process for specific files.
    *   **Parameters:**
        *   `id` (string, required): The ID of the torrent (obtained from `addMagnet`).
        *   `files` (string, required): A comma-separated list of file IDs to select from the torrent.
*   **`DELETE /torrents/delete/{id}`**: Delete a torrent from the user's Real-Debrid account.

### Unrestrict

These endpoints convert restricted hoster links (e.g., from file-sharing sites) into direct download links.

*   **`POST /unrestrict/link`**: Unrestrict a hoster link to get a direct download URL.
    *   **Parameters:**
        *   `link` (string, required): The restricted URL to unrestrict.
    *   **Response (JSON):** Provides the `link` (direct download URL), `filename`, `filesize`, `host`, and other relevant information.

### Downloads

These endpoints manage the user's download history on Real-Debrid.

*   **`GET /downloads`**: Retrieve a list of all files the user has previously downloaded/unrestricted via Real-Debrid.
*   **`DELETE /downloads/delete/{id}`**: Remove a specific download entry from the user's history.

## Error Handling

Real-Debrid API calls return standard HTTP status codes (e.g., 4XX for client errors, 5XX for server errors). Additionally, error responses include a JSON object with:
*   `error` (string): A human-readable error message.
*   `error_code` (integer, optional): A numeric code for programmatic error handling.

Fit-Launcher's backend should map these numeric error codes to appropriate user-facing messages or internal logging.

## Rate Limits

The Real-Debrid API has a rate limit of 250 requests per minute. Exceeding this limit will result in an HTTP 429 error. The backend should implement a robust rate-limiting and exponential back-off strategy to prevent being temporarily blocked.

## Backend Integration Flow (Simplified Sequence Diagram)

```mermaid
sequenceDiagram
    participant FE as Frontend
    participant BE as Backend (Rust)
    participant RD_Auth as Real-Debrid OAuth2 API
    participant RD_API as Real-Debrid REST API

    FE->>BE: User initiates Real-Debrid login
    BE->>RD_Auth: Request Device Code (GET /oauth/v2/device/code)
    RD_Auth-->>BE: Returns device_code, user_code, verification_url, interval, expires_in
    BE->>FE: Display user_code and verification_url for user
    Note over FE,BE: User visits verification_url and enters user_code
    loop Polling for token until authorized or expired
        BE->>RD_Auth: Poll for Access Token (POST /oauth/v2/token with device_code)
        alt User not yet authorized
            RD_Auth-->>BE: Returns error (e.g., "authorization_pending")
        else User authorized
            RD_Auth-->>BE: Returns access_token, refresh_token
            break Loop ends
        end
    end
    BE->>BE: Securely store access_token and refresh_token

    FE->>BE: User provides magnet link for download
    BE->>RD_API: Add Magnet Link (POST /torrents/addMagnet with access_token)
    RD_API-->>BE: Returns torrent info (including file list and torrent ID)
    BE->>FE: Display file selection UI
    FE->>BE: User selects files to download
    BE->>RD_API: Select Files (POST /torrents/selectFiles/{torrent_id} with access_token and selected file IDs)
    RD_API-->>BE: Confirmation (torrent processing starts on RD servers)
    BE->>RD_API: Poll Torrent Info (GET /torrents/info/{torrent_id} with access_token)
    alt Torrent not ready / files not unrestricted
        RD_API-->>BE: Returns torrent info (status: "downloading", "waiting_files", etc.)
    else Torrent ready / files unrestricted
        RD_API-->>BE: Returns torrent info with direct download links for selected files
        break Polling ends
    end
    BE->>FE: Provide direct download links
    FE->>FE: Initiate direct download (e.g., using `tauri-plugin-fs` download capabilities)