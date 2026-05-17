# PRD — Zuro Browser

> **"Navigate without the noise."**
> A cross-platform, privacy-first browser built on Tauri + Rust with Brave-grade ad blocking and Arc-inspired UI/UX.

---

## Table of Contents

1. [Overview](#1-overview)
2. [Problem Statement](#2-problem-statement)
3. [Goals & Non-Goals](#3-goals--non-goals)
4. [Target Users](#4-target-users)
5. [Core Features](#5-core-features)
6. [Technical Architecture](#6-technical-architecture)
7. [UI/UX Design Spec](#7-uiux-design-spec)
8. [Ad-Block Engine Spec](#8-ad-block-engine-spec)
9. [Privacy & Security Model](#9-privacy--security-model)
10. [Data Model](#10-data-model)
11. [IPC Command Reference](#11-ipc-command-reference)
12. [Performance Targets](#12-performance-targets)
13. [Platform Support](#13-platform-support)
14. [Build Phases & Milestones](#14-build-phases--milestones)
15. [Risks & Mitigations](#15-risks--mitigations)
16. [Open Questions](#16-open-questions)

---

## 1. Overview

Zuro is a cross-platform desktop browser built with **Tauri 2.x** (Rust backend) and **React + TypeScript** (frontend). It targets developers, power users, and privacy-conscious individuals who want:

- **Speed** — sub-50ms cold-start, native binary size under 15 MB
- **Privacy** — Brave-grade ad and tracker blocking using the `adblock-rust` crate
- **Productivity** — Arc-style Spaces, vertical sidebar, `⌘K` command palette, auto-archiving tabs
- **Portability** — single Rust codebase targeting Windows, macOS, and Linux

Zuro is **not** a Chromium fork. It uses the OS-native WebView (WebView2 / WKWebView / WebKitGTK) wrapped by Tauri, keeping the binary minimal and delegating web rendering to the system. The ad-block engine sits in Rust at the network layer — before any request reaches the WebView.

---

## 2. Problem Statement

| Pain point | Existing browsers |
|---|---|
| **Memory bloat** | Chrome uses 1–2 GB RAM for 10 tabs. Firefox ~600 MB. |
| **Ads break focus** | Even Brave requires opt-in setup; most users never configure it. |
| **Tab overload** | All major browsers use horizontal tab bars that collapse at 10+ tabs. |
| **Binary size** | Chromium-based browsers ship 150–400 MB installers. |
| **Privacy by default** | Most browsers collect telemetry unless manually disabled. |

Zuro solves all five: native WebView keeps memory per-tab low, ad blocking is on by default with zero setup, the Arc-style vertical sidebar scales to 100+ tabs without collapsing, the Tauri binary is under 15 MB, and no telemetry is collected ever.

---

## 3. Goals & Non-Goals

### Goals

- Block 95%+ of ads and trackers on first launch with zero user configuration
- Cold-start time under 50ms on a 2020 mid-range machine
- Installed binary under 15 MB (Windows), 12 MB (macOS), 10 MB (Linux)
- Support Spaces, tab groups, tab auto-archive, split view, and `⌘K` palette
- Secure local password manager backed by OS keychain
- DNS-over-HTTPS with configurable providers (Cloudflare, NextDNS, custom)
- Full keyboard navigability — no required mouse interaction
- Open-source (MIT or Apache-2.0)

### Non-Goals

- **Not** a Chromium/Firefox fork — Zuro does not implement its own rendering engine
- **Not** a mobile browser — desktop only in v1
- **Not** a sync-to-cloud product in Phase 1 (local-first; sync is Phase 5)
- **Not** a replacement for Chrome extensions ecosystem — WASM plugins only
- **Not** supporting WebExtensions API in v1

---

## 4. Target Users

### Primary — Developer / Power User

Age 20–35. Uses macOS or Linux. Has 40+ tabs open at any time across multiple projects. Uses Arc or Brave already. Cares about keyboard shortcuts, minimal UI, and zero trackers. Willing to use a non-Chrome browser if the UX is better.

**Core need:** A browser that organises tabs into meaningful contexts (work, personal, research) without cluttering the screen.

### Secondary — Privacy-conscious Professional

Age 25–45. Any platform. Switched from Chrome after a data-privacy concern. Uses a VPN and a password manager. Wants ads blocked by default and no opt-in forms on first launch.

**Core need:** A browser that is private out of the box without needing to read documentation.

### Tertiary — Minimalist / Aesthetic User

Drawn to Arc's visual design. Wants a browser that looks designed, not assembled. Will share screenshots; UX is a selling point.

**Core need:** A browser that looks and feels intentional.

---

## 5. Core Features

### 5.1 Spaces (Tab Contexts)

Spaces are isolated browser contexts — each has its own tab list, cookie jar, browsing history, theme accent color, and custom name. Switching Spaces is instant; tabs from other Spaces are not loaded in memory.

- Default Spaces on first launch: **Personal**, **Work**
- User can create, rename, reorder, delete Spaces
- Each Space stores: name, accent color (HSL), pinned tabs, tab archive
- Spaces persist across restarts via SQLite

### 5.2 Vertical Sidebar

The sidebar replaces the traditional horizontal tab bar. It is always visible on the left (width: 220px, collapsible to 48px icon rail).

Sidebar sections (top to bottom):
1. Space selector (colored dots, click to switch)
2. Pinned tabs (always visible, drag to reorder)
3. Open tabs (sorted by last-used, auto-archived after 24h)
4. Bookmarks quick-access
5. Downloads indicator
6. Settings gear (bottom)

Tab items in the sidebar show: favicon, page title (truncated), close button on hover. A subtle visual indicator shows tabs playing audio.

### 5.3 Omnibar

A unified address + search bar at the top. Behaviors:

- Typing a URL → navigate directly
- Typing a query → search via configured engine (default: DuckDuckGo)
- Typing `@bookmarks` → filter bookmarks inline
- Typing `@history` → search history inline
- `Tab` key → cycle through autocomplete suggestions
- Keyboard shortcut: `⌘L` / `Ctrl+L`

### 5.4 Command Palette (`⌘K`)

Inspired by Arc and Linear. Opens a floating modal with fuzzy-search across:

- Open tabs (switch instantly)
- Bookmarks (open or copy URL)
- History (last 500 entries)
- Browser commands (New Space, New Tab, Toggle Sidebar, Open Settings, Clear History...)
- Settings fields (search and jump to any setting)

Keyboard only — no mouse required. `Esc` closes. `↑↓` navigates. `Enter` executes.

### 5.5 Tab Auto-Archive

Tabs not visited in 24 hours are automatically moved to the **Archive** section at the bottom of the sidebar. They are visually dimmed and not loaded in memory. Clicking an archived tab reloads it. Users can configure the archive threshold (off / 24h / 7d).

### 5.6 Split View

Two tabs can be viewed side-by-side within a single Space. Triggered by right-clicking a tab → "Open in Split" or keyboard shortcut `⌘\`. The divider is draggable. Split state persists per Space.

### 5.7 Ad-Block Engine

See Section 8 for full spec. Summary:

- Powered by `adblock-rust` (same crate as Brave)
- On by default, no setup required
- Ships with EasyList + EasyPrivacy + uBlock Origin filters bundled
- Filter lists auto-update every 4 days in background
- Cosmetic filtering hides empty ad container elements
- Block count shown in sidebar footer per-tab

### 5.8 Password Manager

- Stores credentials encrypted in OS keychain via `keyring` crate
- Auto-fill via page script injection on matching domains
- `⌘K` → "Passwords" opens vault UI
- No cloud sync in Phase 1 — local only
- Export to CSV / import from CSV

### 5.9 DNS-over-HTTPS

- Configured in Settings → Privacy → DNS
- Providers: Cloudflare (1.1.1.1), NextDNS, Quad9, Custom URL
- Default: Cloudflare
- Uses `reqwest` with custom DNS resolver via `hickory-dns` (formerly `trust-dns`)

### 5.10 Developer Tools

- Native DevTools via WebView's built-in inspector (`F12`)
- Zuro DevPanel (custom): shows blocked requests count, cosmetic rules applied, DNS query log
- `⌘⌥I` opens the standard WebView inspector

---

## 6. Technical Architecture

### Stack

| Layer | Technology |
|---|---|
| UI framework | React 18 + TypeScript 5 |
| Styling | Tailwind CSS 3.x |
| State management | Zustand 4 |
| Animation | Framer Motion 11 |
| Desktop shell | Tauri 2.x |
| Systems language | Rust (edition 2021) |
| Async runtime | Tokio 1.x |
| HTTP client | reqwest 0.12 (rustls, no OpenSSL) |
| Ad-block engine | adblock 0.8 (adblock-rust) |
| Local database | SQLite via rusqlite 0.31 (bundled) |
| Secrets storage | keyring 2.x |
| Serialization | serde + serde_json |
| DNS resolver | hickory-dns 0.24 |
| Build tooling | Vite 5 (frontend), Cargo (backend) |
| CI/CD | GitHub Actions |
| Distribution | Tauri updater + GitHub Releases |

### Request Intercept Flow

```
User navigates to URL
        │
        ▼
Tauri on_navigation hook fires (Rust)
        │
        ▼
adblock Engine::check_network_urls()
        │
   ┌────┴────┐
 BLOCK     ALLOW
   │          │
   ▼          ▼
Return      Forward to
204 empty   OS WebView
response    (WebView2 / WKWebView / WebKitGTK)
        │
        ▼
Page loads → cosmetic CSS rules injected
via window.__zuro_inject_css(rules)
```

### IPC Pattern

All communication between React and Rust uses Tauri's `invoke()` / `emit()` pattern:

```typescript
// Frontend
const result = await invoke<Tab[]>('get_tabs', { spaceId });

// Backend (Rust)
#[tauri::command]
async fn get_tabs(space_id: String, state: State<'_, AppState>) -> Result<Vec<Tab>, String> {
    state.db.get_tabs(&space_id).await.map_err(|e| e.to_string())
}
```

Events (push from Rust → React) use `emit`:
```rust
app.emit("tab_archived", TabArchivedPayload { tab_id, space_id }).unwrap();
```

---

## 7. UI/UX Design Spec

### Visual Language

- **Typeface:** Inter (UI) / system-ui fallback
- **Icon set:** Tabler Icons (outline)
- **Color system:** HSL-based, Space accent color drives highlight tokens
- **Border radius:** 8px (components), 12px (panels), 16px (modals)
- **Motion:** 150ms ease-out for micro-interactions, 250ms for panel transitions
- **Shadows:** None — flat design only

### Layout

```
┌──────────────────────────────────────────────────────┐
│  [Space dots] [Spaces label]              [⌘K]  [⚙]  │  ← Sidebar header (48px)
├──────────────┬───────────────────────────────────────┤
│              │  [← → ] [🔒 https://...........] [⟳] │  ← Omnibar (44px)
│   Sidebar    ├───────────────────────────────────────┤
│   220px      │                                       │
│              │           WebView                     │
│  ○ Pinned    │           (main content area)         │
│    tab 1     │                                       │
│    tab 2     │                                       │
│              │                                       │
│  ○ Open      │                                       │
│    tab 3     │                                       │
│    tab 4     │                                       │
│              │                                       │
│  ○ Archived  │                                       │
│   (dimmed)   │                                       │
│              │                                       │
│  [🛡 24 blocked] ─────────────────────────────────────│  ← Status bar (28px)
└──────────────┴───────────────────────────────────────┘
```

### Keyboard Shortcuts

| Action | macOS | Windows / Linux |
|---|---|---|
| New tab | `⌘T` | `Ctrl+T` |
| New Space | `⌘⇧N` | `Ctrl+Shift+N` |
| Command palette | `⌘K` | `Ctrl+K` |
| Focus omnibar | `⌘L` | `Ctrl+L` |
| Close tab | `⌘W` | `Ctrl+W` |
| Toggle sidebar | `⌘⇧S` | `Ctrl+Shift+S` |
| Split view | `⌘\` | `Ctrl+\` |
| Cycle Spaces | `⌘1–9` | `Ctrl+1–9` |
| Developer tools | `⌘⌥I` | `F12` |
| Toggle ad-block | `⌘⇧A` | `Ctrl+Shift+A` |
| Open passwords | `⌘⇧P` | `Ctrl+Shift+P` |

### Onboarding Flow (First Launch)

1. **Welcome screen** — Zuro name + tagline, single CTA "Get started"
2. **Default search engine** — DuckDuckGo (selected), Google, Bing, Brave Search, Custom
3. **Import** — Chrome / Firefox bookmarks (optional, skippable)
4. **All done** — ad blocking is already on, Spaces already created. No more setup.

Total steps: 3. Total required clicks: 1 (just hit Enter through all).

---

## 8. Ad-Block Engine Spec

### Library

`adblock = "0.8"` — the `adblock-rust` crate maintained by the Brave team. Implements:

- uBlock Origin filter syntax
- EasyList / EasyPrivacy / AdGuard Base filter parsing
- Cosmetic filter matching (`##` element hiding rules)
- Scriptlet injection blocking
- `$csp` and `$redirect` rule support

### Bundled Filter Lists (shipped in binary)

| List | Description | Size (compressed) |
|---|---|---|
| EasyList | Main ad-blocking list | ~300 KB |
| EasyPrivacy | Tracker blocking | ~150 KB |
| uBlock Origin filters | Extra coverage | ~200 KB |
| Peter Lowe's list | Ad servers / tracking | ~50 KB |

Total bundled: ~700 KB compressed, embedded via `include_bytes!()` at compile time.

### Filter List Update Mechanism

- On launch, check `~/.zuro/filters/last_updated` timestamp
- If older than 4 days, spawn background `tokio::task` to download fresh lists
- Download from CDN (jsDelivr mirror of EasyList repo) via `reqwest`
- Parse and hot-reload into engine without restart
- Failed update → continue using cached lists silently

### Network Intercept

```rust
// src-tauri/src/adblock/interceptor.rs

pub fn should_block(
    engine: &Engine,
    url: &str,
    source_url: &str,
    resource_type: ResourceType,
) -> bool {
    let request = Request::new(url, source_url, resource_type.as_str())
        .unwrap_or_default();
    engine.check_network_request(&request).matched
}
```

This runs synchronously on every navigation and sub-resource load. Benchmark target: < 0.1ms per check.

### Cosmetic Filter Injection

After a page load completes, Zuro injects a `<style>` tag containing all cosmetic rules matching the current domain:

```rust
let rules: Vec<String> = engine
    .hidden_class_id_selectors(classes, ids, &[hostname.to_string()])
    .unwrap_or_default();

let css = rules.join(",\n") + " { display: none !important; }";
webview.eval(&format!("__zuro_inject_css(`{css}`)"))?;
```

### Stats

Per-tab block count is tracked in a `HashMap<TabId, u32>` in `AppState`. Emitted to frontend via event on each block. Shown in sidebar status bar.

### User Controls

- **Global toggle** — disable/enable all blocking (persists to settings)
- **Per-site allowlist** — right-click page → "Disable Zuro blocking for this site"
- **Custom filter rules** — Settings → Ad-block → Custom rules textarea (uBlock syntax)
- **Filter list manager** — Settings → Ad-block → Lists (toggle individual lists, add custom URLs)

---

## 9. Privacy & Security Model

### Principles

1. **No telemetry, ever.** Zuro phones home for nothing. No analytics, no crash reporting sent to servers. Crash logs are local only.
2. **Local-first.** All data (history, bookmarks, passwords, settings) lives in `~/.zuro/` — never uploaded without explicit user action.
3. **Zero accounts required.** Zuro works fully offline with no login.

### Data Collected

| Data | Stored | Sent externally |
|---|---|---|
| Browsing history | SQLite, local | Never |
| Bookmarks | SQLite, local | Never |
| Passwords | OS keychain | Never |
| Filter list updates | Downloaded | One-way HTTP GET |
| Crash logs | Local file | Never |
| Usage analytics | None collected | N/A |

### Certificate Validation

- `rustls` (pure-Rust TLS) handles all HTTPS connections from Zuro's Rust layer
- The OS WebView uses its native TLS stack for page rendering
- HSTS enforcement via local HSTS preload list (Chromium's list, bundled)

### Content Security

- `javascript:` URLs blocked by default
- Mixed content (HTTP on HTTPS page) blocked and flagged in omnibar
- `blob:` and `data:` URL navigations require user confirmation for downloads

---

## 10. Data Model

### SQLite Schema

```sql
-- Spaces
CREATE TABLE spaces (
  id          TEXT PRIMARY KEY,
  name        TEXT NOT NULL,
  accent_hsl  TEXT NOT NULL DEFAULT '262 80% 60%',
  position    INTEGER NOT NULL DEFAULT 0,
  created_at  INTEGER NOT NULL
);

-- Tabs
CREATE TABLE tabs (
  id          TEXT PRIMARY KEY,
  space_id    TEXT NOT NULL REFERENCES spaces(id) ON DELETE CASCADE,
  url         TEXT NOT NULL,
  title       TEXT NOT NULL DEFAULT '',
  favicon_url TEXT,
  is_pinned   INTEGER NOT NULL DEFAULT 0,
  is_archived INTEGER NOT NULL DEFAULT 0,
  last_visited INTEGER NOT NULL,
  position    INTEGER NOT NULL DEFAULT 0,
  created_at  INTEGER NOT NULL
);

-- History
CREATE TABLE history (
  id          INTEGER PRIMARY KEY AUTOINCREMENT,
  space_id    TEXT NOT NULL,
  url         TEXT NOT NULL,
  title       TEXT NOT NULL DEFAULT '',
  visited_at  INTEGER NOT NULL
);
CREATE INDEX history_visited ON history(visited_at DESC);
CREATE INDEX history_url ON history(url);

-- Bookmarks
CREATE TABLE bookmarks (
  id          TEXT PRIMARY KEY,
  space_id    TEXT,
  url         TEXT NOT NULL,
  title       TEXT NOT NULL,
  folder      TEXT,
  created_at  INTEGER NOT NULL
);

-- Settings
CREATE TABLE settings (
  key   TEXT PRIMARY KEY,
  value TEXT NOT NULL
);

-- Block stats
CREATE TABLE block_stats (
  tab_id      TEXT NOT NULL,
  url         TEXT NOT NULL,
  blocked_at  INTEGER NOT NULL,
  list_source TEXT
);
```

### Settings Keys

| Key | Type | Default |
|---|---|---|
| `default_search_engine` | string | `duckduckgo` |
| `adblock_enabled` | bool | `true` |
| `adblock_allowlist` | JSON array of strings | `[]` |
| `tab_archive_hours` | integer | `24` |
| `sidebar_collapsed` | bool | `false` |
| `dns_over_https_provider` | string | `cloudflare` |
| `dns_over_https_custom_url` | string | `""` |
| `theme` | string | `system` |
| `new_tab_page` | string | `zuro://newtab` |

---

## 11. IPC Command Reference

### Navigation

| Command | Params | Returns |
|---|---|---|
| `navigate` | `{ tab_id, url }` | `()` |
| `go_back` | `{ tab_id }` | `()` |
| `go_forward` | `{ tab_id }` | `()` |
| `reload` | `{ tab_id }` | `()` |
| `stop_loading` | `{ tab_id }` | `()` |

### Tabs

| Command | Params | Returns |
|---|---|---|
| `create_tab` | `{ space_id, url? }` | `Tab` |
| `close_tab` | `{ tab_id }` | `()` |
| `get_tabs` | `{ space_id }` | `Vec<Tab>` |
| `pin_tab` | `{ tab_id }` | `()` |
| `archive_tab` | `{ tab_id }` | `()` |
| `reorder_tabs` | `{ space_id, ordered_ids }` | `()` |

### Spaces

| Command | Params | Returns |
|---|---|---|
| `create_space` | `{ name, accent_hsl? }` | `Space` |
| `get_spaces` | `{}` | `Vec<Space>` |
| `update_space` | `{ id, name?, accent_hsl? }` | `Space` |
| `delete_space` | `{ id }` | `()` |
| `switch_space` | `{ id }` | `()` |

### Ad-block

| Command | Params | Returns |
|---|---|---|
| `get_block_stats` | `{ tab_id }` | `BlockStats` |
| `toggle_adblock` | `{ enabled }` | `()` |
| `add_to_allowlist` | `{ domain }` | `()` |
| `remove_from_allowlist` | `{ domain }` | `()` |
| `get_filter_lists` | `{}` | `Vec<FilterList>` |
| `update_filter_lists` | `{}` | `()` |

### Events (Rust → React)

| Event | Payload |
|---|---|
| `tab_updated` | `{ tab_id, url, title, is_loading }` |
| `tab_archived` | `{ tab_id, space_id }` |
| `request_blocked` | `{ tab_id, url, list_source }` |
| `filter_lists_updated` | `{ updated_at }` |
| `download_started` | `{ id, url, filename }` |
| `download_completed` | `{ id, path }` |

---

## 12. Performance Targets

| Metric | Target | Measurement method |
|---|---|---|
| Cold start to interactive | < 50ms | Tauri `ready` event timestamp |
| Binary size (Windows) | < 15 MB | `cargo build --release` output |
| Binary size (macOS) | < 12 MB | `cargo build --release` output |
| Memory (10 tabs, idle) | < 200 MB | macOS Activity Monitor |
| Ad-block check latency | < 0.1ms per request | Rust benchmark (`cargo bench`) |
| Tab switch time | < 16ms (1 frame) | JS `performance.now()` diff |
| Filter list parse (700 KB) | < 200ms | Rust benchmark on cold start |
| SQLite query (history search) | < 5ms | Rust benchmark |
| Command palette open | < 50ms | JS `performance.now()` diff |

---

## 13. Platform Support

| Platform | WebView engine | Min version | Build target |
|---|---|---|---|
| Windows | WebView2 (Edge Chromium) | Windows 10 21H2 | `x86_64-pc-windows-msvc` |
| macOS | WKWebView | macOS 12 Monterey | `x86_64-apple-darwin`, `aarch64-apple-darwin` (universal) |
| Linux | WebKitGTK 4.1 | Ubuntu 22.04 / Fedora 37 | `x86_64-unknown-linux-gnu` |

### Distribution

| Platform | Format |
|---|---|
| Windows | NSIS installer (`.exe`) + MSI |
| macOS | `.dmg` (universal binary) |
| Linux | `.AppImage`, `.deb`, `.rpm` |

Auto-updater via `tauri-plugin-updater`, checking GitHub Releases JSON endpoint.

---

## 14. Build Phases & Milestones

### Phase 1 — Shell (Weeks 1–3)

**Goal:** A Tauri window that renders web pages with a functional omnibar.

- [ ] Initialize Tauri 2.x project with React + TypeScript + Vite
- [ ] Configure Tailwind CSS + Zustand
- [ ] Implement omnibar with URL navigation and DuckDuckGo search
- [ ] `go_back`, `go_forward`, `reload` IPC commands
- [ ] Vertical sidebar scaffold (static layout, no tabs yet)
- [ ] SQLite setup with `spaces` and `tabs` schema
- [ ] Basic tab creation and switching (one Space)

**Exit criteria:** Can navigate to websites, go back/forward, and the sidebar renders.

---

### Phase 2 — Ad-block (Weeks 4–5)

**Goal:** Requests are blocked at the network layer before the WebView renders them.

- [ ] Add `adblock` crate to `Cargo.toml`
- [ ] Bundle EasyList + EasyPrivacy as `include_bytes!()` at compile time
- [ ] Implement `interceptor.rs` — hook into Tauri navigation events
- [ ] Inject cosmetic CSS rules post-load
- [ ] Per-tab block count tracking + sidebar indicator
- [ ] Per-site allowlist (add/remove domain)
- [ ] Global toggle (enable / disable all blocking)
- [ ] Filter list auto-updater (background `tokio::task`, every 4 days)

**Exit criteria:** Block count appears in sidebar. Visiting a news site shows 20+ requests blocked. Cosmetic rules hide empty ad containers.

---

### Phase 3 — Arc UI (Weeks 6–9)

**Goal:** Full Arc-inspired sidebar UX with Spaces, tab groups, archive, and command palette.

- [ ] Spaces: create, rename, delete, switch (persisted to SQLite)
- [ ] Space accent color customization (HSL color picker)
- [ ] Tabs: pinned section, open tabs section, archived section
- [ ] Tab auto-archive (`tokio::time` timer, configurable threshold)
- [ ] Drag-to-reorder tabs via `@dnd-kit/core`
- [ ] Sidebar collapse to 48px icon rail
- [ ] `⌘K` command palette (fuzzy search: tabs, bookmarks, history, commands)
- [ ] Split view (two WebViews side by side, draggable divider)
- [ ] Tab audio indicator (WebView media state API)
- [ ] Framer Motion transitions for sidebar and palette

**Exit criteria:** Can create 3 Spaces, each with their own tabs, switch instantly between them, and find any open tab in under 2 keystrokes via `⌘K`.

---

### Phase 4 — Privacy (Weeks 10–12)

**Goal:** DNS-over-HTTPS, password manager, fingerprint protection.

- [ ] DNS-over-HTTPS via `hickory-dns` (Cloudflare, NextDNS, Quad9, custom)
- [ ] Password manager: store, retrieve, autofill (via `keyring` crate)
- [ ] Password vault UI in command palette
- [ ] Import passwords from CSV
- [ ] Export passwords to encrypted CSV
- [ ] HTTPS-upgrade enforcement (HTTP → HTTPS redirect)
- [ ] Blocked request log (DevPanel → "Zuro Privacy" tab)
- [ ] Privacy score badge in omnibar (based on tracker count)

**Exit criteria:** Passwords autofill on returning to a site. DNS queries route through DoH. HTTPS is enforced on sites that support it.

---

### Phase 5 — Polish & Extensions (Weeks 13–18)

**Goal:** Extension support, sync, auto-updater, public release.

- [ ] WASM extension runner (sandboxed, WASI)
- [ ] Extension manager UI (install from `.zuro-ext` package)
- [ ] E2E encrypted sync via user-provided S3 / Cloudflare R2 bucket
- [ ] Tauri auto-updater integration
- [ ] GitHub Actions CI (build + sign for Win/Mac/Linux)
- [ ] Code-signed macOS `.dmg` (Apple Developer Program)
- [ ] Onboarding flow (3-step wizard, import from Chrome/Firefox)
- [ ] Custom new tab page (`zuro://newtab` — clock, bookmarks, recent tabs)
- [ ] Performance audit and memory profiling
- [ ] Public beta release on GitHub Releases

**Exit criteria:** Installable from a signed binary on all three platforms. Auto-updater delivers patches. Public GitHub repo is live.

---

## 15. Risks & Mitigations

| Risk | Likelihood | Impact | Mitigation |
|---|---|---|---|
| OS WebView inconsistencies across platforms | High | High | Abstract WebView API behind a Tauri plugin; test on all three platforms in CI |
| `adblock-rust` API changes between versions | Medium | Medium | Pin to `adblock = "0.8"`, review changelogs before upgrading |
| WebView2 not installed on older Windows machines | Medium | High | Tauri NSIS installer includes WebView2 bootstrapper |
| WebKitGTK version fragmentation on Linux | High | Medium | Target Ubuntu 22.04 LTS as minimum; test AppImage on Fedora 38 |
| Password autofill breaks on SPAs | Medium | Medium | Inject autofill script on every navigation and DOM mutation |
| Filter list CDN downtime | Low | Low | Ship bundled lists; update is best-effort, not blocking |
| Binary size creep past 15 MB | Medium | Low | Track with `cargo bloat` in CI; strip symbols in release builds |
| Tauri 2.x API instability (still beta at time of writing) | Medium | High | Track Tauri 2.x RC releases; pin to a release candidate |

---

## 16. Open Questions

1. **Search engine default** — Should the default be DuckDuckGo or Brave Search? DuckDuckGo has more brand recognition; Brave Search is more privacy-preserving. Decision needed before Phase 1.

2. **Sync backend** — Phase 5 proposes user-provided S3/R2 bucket. Should Zuro offer a hosted sync option (requires server infrastructure and a privacy policy)? Or keep it fully self-hosted?

3. **Extension format** — WASM extensions are sandboxed but incompatible with the Chrome extension ecosystem. Is a compatibility layer for a subset of WebExtensions APIs worth the complexity? Deferring to post-v1.

4. **macOS notarization** — Requires an Apple Developer Program account ($99/year). Who holds this account? Needed before Phase 5 ships.

5. **Telemetry opt-in** — Should Zuro offer a voluntary, anonymous crash reporter (e.g., Sentry with user opt-in on first launch)? Current position is no telemetry ever, but this makes debugging production issues harder.

6. **Tab session restore** — Should Zuro restore all tabs from the previous session on launch, or open fresh? Arc opens a clean session. Chrome restores. User preference or opinionated default?

7. **Mobile roadmap** — Tauri does not yet support iOS/Android in a stable release. Is mobile a v2 goal? If so, which platform first?

---

*Document version: 1.0.0 — Last updated: May 2026*
*Author: Ansh Jadhav — Zuro Browser Project*
