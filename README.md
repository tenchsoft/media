<div align="center">

# Tench Media

**A four-app media suite, built in 100% Rust.**

View · Pixel Design · Player · Composer — from instant image browsing to multi-track video editing, all running locally.

[![Language: Rust](https://img.shields.io/badge/Language-Rust-dea584.svg)](https://www.rust-lang.org/)
[![Framework: Tauri 2](https://img.shields.io/badge/Framework-Tauri_2-FFC140.svg)](https://v2.tauri.app/)
[![License: UNLICENSED](https://img.shields.io/badge/License-UNLICENSED-red.svg)](#license)
[![Status: Preview](https://img.shields.io/badge/Status-Preview-orange.svg)](#roadmap)
[![Pricing: $1/mo](https://img.shields.io/badge/Pricing-%241%2Fmo-1ca096.svg)](https://tenchsoft.com/pricing)

</div>

---

## Overview

Tench Media bundles four media tools — an image viewer, an image editor, a media player, and a multi-track video editor — all built in Rust with Tauri 2 and a self-hosted UI framework. No Electron, no JavaScript, no cloud uploads.

Benchmarked against established tools (Honeyview, Photoshop, PotPlayer, Premiere Pro) but reimagined around three Tench principles: local-first, AI-assisted, fast.

## Products

| | Product | Benchmarked against | Description |
|---|---|---|---|
| 🖼️ | **View** | Honeyview | Instant image viewer for 100k+ libraries with EXIF, tags, AI classification |
| 🎨 | **Pixel Design** | Adobe Photoshop | Image editor with canvas, layers, brushes, blend modes, AI fill |
| 🎬 | **Player** | PotPlayer | Media player with playlists, subtitles, GIF capture, AI scene summaries |
| 🎞️ | **Composer** | Adobe Premiere Pro | Multi-track video editor with timeline, captions, transitions, export pipeline |

## Features

- **Format coverage** — PSD/PNG/JPEG/WebP/TIFF, MP4/WebM/MKV, SRT/ASS/VTT, RAR/7z/ZIP archives.
- **Hardware acceleration** — GPU decode/playback, wgpu rendering.
- **Local-first AI** — background removal, upscaling, generative fill, auto-tagging, scene detection — all via Tench Engine.
- **EXIF / IPTC / XMP** — full metadata display and batch editing.
- **GIF & screenshot capture** — any frame, any region, configurable frame rate.
- **Cross-platform** — Windows, macOS, Linux.

## Architecture

```
apps/<product>/src-tauri/        Product shells (Tauri 2)
crates/media-core/               Shared media types & codecs glue
crates/image-core/               Image pipeline primitives
crates/image-runtime/            View + Pixel Design engine
crates/pixel-core/               Pixel Design editing model (layers, brushes, masks)
crates/media-runtime/            Player + Composer engine
crates/media-playback/           Hardware-accelerated decode/playback
crates/subtitle-core/            SRT / ASS / VTT reader/writer
crates/composer-core/            Multi-track timeline, transitions, export
crates/storage-core/             Local persistence + AES-GCM encryption
crates/fs-core/                  File-system access policy
crates/engine-core/              Tench Engine client
crates/tench-ui/                 Self-built widget framework
crates/tench-ui-test/            Headless E2E harness
tools/architecture-guard/        Repo structure enforcement
```

## Build

```bash
cargo check --workspace --locked
cargo build --workspace --locked
cargo test --workspace --locked
cargo run --locked -p view    # or: pixel-design, player, composer
```

## Roadmap

- [x] View — 100k+ image library, EXIF, slideshow
- [x] Pixel Design — canvas, layers, brushes, AI fill
- [x] Player — all formats, subtitles, GIF capture
- [x] Composer — multi-track timeline, captions, transitions
- [ ] Mobile companion (gallery + viewer)
- [ ] AI upscaling & generative fill in Pixel Design
- [ ] Composer: 4K proxy editing

## Pricing

- **$1 / month per device** — every Tench Media + Tench Office app.
- Bulk pricing for 5+ devices.

→ https://tenchsoft.com/pricing

## License

UNLICENSED — source available for review, binary distribution requires a subscription.

## Sister Projects

- **[Tench Office](https://github.com/tenchsoft/office)** — Docs / Sheets / Slides / Kodocs
- **[Tench Authoring](https://github.com/tenchsoft/authoring)** — Story / Universe
- **[Tench Knowledge](https://github.com/tenchsoft/knowledge)** — Research / Study
- **[Tench Code](https://github.com/tenchsoft/code)** — AI-augmented code editor
- **[tenchsoft.com](https://tenchsoft.com)** — account, license, downloads
