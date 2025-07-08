# Plan: Replace All Downloading with Real-Debrid

## 1. Remove/Refactor Existing Download Logic

### Frontend (JS/JSX)
- Remove or refactor all torrent/aria2/DDL logic and UI:
  - [`src/pages/Settings-01/Settings-Categories/Torrenting/Torrenting.jsx`](src/pages/Settings-01/Settings-Categories/Torrenting/Torrenting.jsx)
  - [`src/pages/Settings-01/Settings.jsx`](src/pages/Settings-01/Settings.jsx)
  - [`src/pages/Downloads-01/Downloads-Page.jsx`](src/pages/Downloads-01/Downloads-Page.jsx)
  - [`src/pages/Download-Game-UUID-01/Download-Game-UUID.jsx`](src/pages/Download-Game-UUID-01/Download-Game-UUID.jsx)
  - [`src/components/functions/dataStoreGlobal.jsx`](src/components/functions/dataStoreGlobal.jsx) (remove torrent-related stores and helpers)
  - [`src/Pop-Ups/Download-PopUp/Download-PopUp.jsx`](src/Pop-Ups/Download-PopUp/Download-PopUp.jsx) (replace torrent logic with Real-Debrid logic)
  - All references to `torrentIdx`, `magnetlink`, `aria2`, and related download management in other components/pages.

### Backend (Rust)
- Remove or refactor all code in:
  - [`src-tauri/local-crates/fit-launcher-aria2/`](src-tauri/local-crates/fit-launcher-aria2/)
  - [`src-tauri/local-crates/fit-launcher-torrent/`](src-tauri/local-crates/fit-launcher-torrent/)
  - [`src-tauri/local-crates/fit-launcher-ddl/`](src-tauri/local-crates/fit-launcher-ddl/)
  - Remove all `aria2_*`, `torrent_*`, `ddl_*` commands and related structs/functions from the Tauri backend.
  - Remove torrent/aria2 config from [`src-tauri/local-crates/fit-launcher-config/`](src-tauri/local-crates/fit-launcher-config/).

---

## 2. Integrate Real-Debrid

### Backend
- Create a new crate (e.g. `fit-launcher-real-debrid`) or module for Real-Debrid API integration.
- Implement endpoints for:
  - OAuth2 authentication (see [`doc/real-debrid.md`](doc/real-debrid.md) for device code flow).
  - Adding magnet links/torrents: `POST /torrents/addMagnet`, `PUT /torrents/addTorrent`
  - Selecting files: `POST /torrents/selectFiles/{id}`
  - Checking status: `GET /torrents/info/{id}`, `GET /torrents`
  - Downloading unrestricted links: `POST /unrestrict/link`
  - Managing downloads: `GET /downloads`, `DELETE /downloads/delete/{id}`

### Frontend
- Update download UI to only support Real-Debrid:
  - Add authentication flow for Real-Debrid (device code or OAuth2).
  - Replace all download triggers to use Real-Debrid endpoints.
  - Show only Real-Debrid download progress/status.
  - Remove all UI for torrent/aria2/DDL settings and progress.

---

## 3. Update Application Flow
- All download requests should route through the new Real-Debrid backend.
- Refactor or remove logic assuming multiple download backends.
- Update or remove settings/config screens related to legacy downloaders.

---

## 4. Cleanup and Documentation
- Remove unused dependencies and configuration.
- Update documentation to reflect the new download flow and Real-Debrid authentication.
- Test all flows for regressions.

---

## 5. Risks & Considerations
- Real-Debrid API limits and error handling.
- User authentication and token management.
- Removal of legacy code may break unrelated features if tightly coupled.

---

## 6. Reference: Real-Debrid API

See [`doc/real-debrid.md`](doc/real-debrid.md) for:
- Authentication (OAuth2/device code)
- Torrent/magnet link handling
- File selection and download management
- Error codes and API limits

---

## Diagram: New Download Flow

```mermaid
flowchart TD
  UserUI[User Download Request]
  RealDebridUI[Real-Debrid Auth & Link Input]
  BackendAPI[Backend Real-Debrid API]
  RealDebrid[Real-Debrid Service]
  DirectLink[Direct Download Link]
  UserUI --> RealDebridUI --> BackendAPI --> RealDebrid --> DirectLink --> UserUI