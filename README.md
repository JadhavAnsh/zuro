# Zuro Browser

**Navigate without the noise.**

Zuro Browser is a cross-platform desktop web browser designed around three priorities: speed, privacy, and focus. It combines a native desktop shell, system WebView rendering, and built-in ad and tracker blocking with a workspace-oriented browsing experience inspired by modern productivity tools.

This repository represents the Zuro desktop app: a browser for people who want a cleaner web, less interface clutter, and stronger privacy defaults without relying on heavy Chromium-based distributions.

## What Zuro Is

Zuro is a desktop browser built with:

- **Tauri + Rust** for the native application layer
- **React + TypeScript** for the interface
- **OS-native WebView engines** for page rendering
- **Rust-based network filtering** for built-in ad and tracker blocking

Rather than shipping a full browser engine inside the app, Zuro uses the rendering technology already available on the operating system:

- **Windows:** WebView2
- **macOS:** WKWebView
- **Linux:** WebKitGTK

That approach keeps the desktop app lightweight while still delivering a full browsing experience.

## Product Direction

Zuro is built for users who want a browser that feels intentional.

It is aimed at:

- **Developers and power users** managing many tabs across different contexts
- **Privacy-conscious users** who want ad and tracker blocking enabled by default
- **Design-sensitive users** who care about a calmer, more organized desktop interface

The product focuses on reducing noise in two places:

- **On the web:** by blocking ads, trackers, and intrusive page elements
- **In the browser UI:** by replacing tab sprawl with structured spaces and a vertical workflow

## Core Desktop Experience

### Spaces

Zuro organizes browsing into **Spaces**. Each Space acts like its own context with separate tabs, history, cookies, pinned items, and visual identity.

Typical uses include:

- Work
- Personal
- Research
- Client-specific sessions

This makes it easier to keep browsing contexts separated without juggling multiple browser profiles or windows.

### Vertical Sidebar

Instead of a traditional horizontal tab strip, Zuro uses a **vertical sidebar** that stays readable even with large numbers of tabs.

The sidebar is designed to give quick access to:

- Space switching
- Pinned tabs
- Active tabs
- Archived tabs
- Bookmarks
- Downloads and browser status

This layout is intended to scale better for heavy desktop use than conventional tab bars.

### Omnibar

Zuro provides a unified **address and search bar** that handles:

- Direct URL navigation
- Search queries
- Inline access to bookmarks
- Inline access to history

The goal is a fast, keyboard-friendly entry point for both navigation and recall.

### Command Palette

The desktop app includes a **command palette** for quick browser actions and navigation. It is designed to let users search across:

- Open tabs
- Bookmarks
- History
- Browser commands
- Settings destinations

This keeps common actions accessible without digging through menus.

### Tab Auto-Archive

To reduce tab overload, Zuro can automatically move inactive tabs into an **archive state**. Archived tabs remain visible but do not stay actively loaded in memory until reopened.

This supports a cleaner workspace while preserving session continuity.

### Split View

Zuro supports **side-by-side tab viewing** within the same Space. This is useful for comparing pages, referencing documentation while working, or keeping two related workflows visible at once.

## Privacy by Default

Privacy is a product principle, not an optional mode.

Zuro is designed so that important protections are enabled from the start:

- **Built-in ad and tracker blocking**
- **No telemetry by default**
- **Local-first data storage**
- **No required account**
- **Configurable DNS-over-HTTPS**

The intended experience is that users do not need to install multiple extensions or work through setup guides just to get baseline privacy protections.

## Ad and Tracker Blocking

Zuro includes a native filtering engine powered by the Rust ad-blocking ecosystem used in privacy-first browsers.

Product goals for blocking include:

- Blocking ads and trackers immediately on first launch
- Filtering requests before they reach the rendering layer
- Applying cosmetic cleanup to remove empty ad containers
- Exposing block visibility inside the browser UI
- Supporting site-level allowlisting and custom rules

The result is meant to be a quieter, faster browsing experience with less page clutter and fewer hidden network requests.

## Security and Local Ownership

Zuro is built around a local-first desktop model:

- Browsing data is stored on the user’s device
- Passwords are intended to use the operating system keychain
- Browser settings remain under user control
- The app does not depend on a mandatory cloud account

This keeps the browser usable offline and reduces the amount of personal activity that depends on third-party services.

## Design Philosophy

Zuro is not trying to imitate a traditional browser with a new skin. Its design direction is based on a few clear ideas:

- **Focused, not crowded**
- **Private, not configurable only after setup**
- **Fast, not oversized**
- **Structured, not tab-chaotic**

The interface is meant to feel more like a deliberate desktop tool than a generic browser shell.

## Why It Exists

Many browsers force a tradeoff between performance, privacy, and usability:

- Some are fast but visually cluttered
- Some are private but require manual setup
- Some are polished but resource-heavy

Zuro is meant to close that gap with a desktop browser that feels modern, stays lightweight, and treats privacy and organization as defaults instead of upgrades.

## Platform Scope

Zuro is a **desktop-only** product.

Target platforms:

- Windows
- macOS
- Linux

The app is designed to use each platform’s native web rendering engine while keeping a consistent Zuro interface and browsing model across systems.

## Repository Context

This repository is the home of the **Zuro desktop application**. Its purpose is to define and build the browser itself: the native shell, interface, browsing model, privacy controls, and core desktop experience.

If you are viewing this project for the first time, the main thing to know is simple:

**Zuro is a privacy-first desktop browser focused on clean navigation, structured tab management, and a lighter, less distracting web experience.**
