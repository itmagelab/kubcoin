# KubCoin Project Structure

This project is a landing page for the **KubCoin** Telegram bot (a tool for personal finance management). The project is built using the **Rust** + **Yew (WebAssembly)** stack, compiled and run via the **Trunk** build tool.

---

## Directory Structure

### Root Configuration Files
* [Cargo.toml](file:///Users/wilful/Git/kubcoin/Cargo.toml) — Rust dependencies configuration, including Yew, Yew Router, Serde, and compile optimizations for WebAssembly.
* [Trunk.toml](file:///Users/wilful/Git/kubcoin/Trunk.toml) — Trunk build tool configuration (local server settings on port `8080` and asset path mapping).
* [index.html](file:///Users/wilful/Git/kubcoin/index.html) — HTML template for the page. Contains SEO meta-tags, Yandex.Metrika tracking, Tailwind CSS integration (via CDN), fonts, and a `<main id="main-content">` element where the Yew application is mounted.
* [README.md](file:///Users/wilful/Git/kubcoin/README.md) — general project information and run instructions.

---

### Source Code (`src/`)
All application logic, UI, and localization implementation resides in the [src](file:///Users/wilful/Git/kubcoin/src) directory.

* [src/main.rs](file:///Users/wilful/Git/kubcoin/src/main.rs) — application entry point. Initializes logging via `tracing`, configures the root `App` component, sets up routing, and initiates the `LanguageProvider`.
* [src/router.rs](file:///Users/wilful/Git/kubcoin/src/router.rs) — routing module. Defines the `Route` enum supporting localized paths (`/ru`, `/en`) and default redirects.
* [src/i18n.rs](file:///Users/wilful/Git/kubcoin/src/i18n.rs) — root internationalization module exporting the provider and related types.

#### Localization (`src/i18n/`)
The project fully supports multi-language capabilities (Russian and English).
* [src/i18n/types.rs](file:///Users/wilful/Git/kubcoin/src/i18n/types.rs) — defines the `Language` enum and includes browser language auto-detection logic.
* [src/i18n/content.rs](file:///Users/wilful/Git/kubcoin/src/i18n/content.rs) — data structures for deserializing translation files from YAML at compile time using `include_str!`.
* [src/i18n/provider.rs](file:///Users/wilful/Git/kubcoin/src/i18n/provider.rs) — context component `LanguageProvider` and the `use_language` hook to access the current language and translation strings. Saves the user's choice to `LocalStorage`.

#### UI Components (`src/html/` and `src/html.rs`)
* [src/html.rs](file:///Users/wilful/Git/kubcoin/src/html.rs) — major sections of the landing page:
  - `Header` — website header with application screenshot and calls to action.
  - `Body` — container hosting various sections of the page.
  - `Features` — benefits and main features of the bot.
  - `Chats` — interactive chat flow examples showing bot dialogues.
  - `Pricing` — pricing plans (Free and Premium).
  - `Security` — information about data safety and privacy.
  - `QA` — frequently asked questions (FAQ) with interactive spoilers.
  - `Usage` — usage statistics block.
  - `Footer` — page footer containing copyright details.
* [src/html/button.rs](file:///Users/wilful/Git/kubcoin/src/html/button.rs) — reusable button links to navigate to Telegram (group, channel, or bot).
* [src/html/icon.rs](file:///Users/wilful/Git/kubcoin/src/html/icon.rs) — SVG icons (e.g., the KubCoin coin icon).
* [src/html/language_switcher.rs](file:///Users/wilful/Git/kubcoin/src/html/language_switcher.rs) — UI language toggle switcher (RU/EN).

---

### Static Assets (`static/`)
Contains resources copied by Trunk into the final release build.
* [static/i18n/ru.yaml](file:///Users/wilful/Git/kubcoin/static/i18n/ru.yaml) — Russian language interface translations (structured data).
* [static/i18n/en.yaml](file:///Users/wilful/Git/kubcoin/static/i18n/en.yaml) — English language interface translations.
* [static/content.yaml](file:///Users/wilful/Git/kubcoin/static/content.yaml) — supplementary configuration content data.
* **images/** — screenshots and graphic assets used throughout the page.
* **robots.txt**, **sitemap.xml** — search engine crawler configuration files (SEO).

---

### Documentation (`doc/`)
The [doc](file:///Users/wilful/Git/kubcoin/doc) directory contains working guidelines and materials:
* `conventions.md` — coding standards and design conventions.
* `workflow.md` — team workflow instructions.
* `vision.md` — product vision and project overview.
* `tasklist.md` — current project task tracking checklist.

---

## Key Development Commands

* **Run local development server**:
  ```bash
  trunk serve
  ```
  The project will be available at `http://127.0.0.1:8080/` with live reload support on file changes.

* **Build for production**:
  ```bash
  trunk build --release
  ```
  Compiled output files (WASM binary, JS loader, index HTML, and assets) will be placed in the `dist/` directory.
