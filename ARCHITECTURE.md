# Repository Architecture

This repo is organized around shared foundations first, product shells second.
The detailed product plans live in `plans/`.

## Layers

```text
apps/*              Thin product app shells (Tauri 2)
crates/*-core       Shared Rust domain and platform contracts
crates/tench-ui     Self-built retained-mode UI framework (Vello + Parley + wgpu)
crates/tench-ui-test  Headless test harness for UI E2E
tools/*             Repo automation and CI entrypoints
```

## Crate Dependency Graph

```text
tench-ui ──────── ui-automation-core
tench-ui-test ─── tench-ui, ui-automation-core

media-core ────── fs-core
image-core ────── media-core
image-runtime ─── image-core, pixel-core, storage-core, tench-ui
pixel-core (독립)

media-runtime ─── media-core, shared-types, storage-core, subtitle-core
media-playback (독립)
subtitle-core (독립)
composer-core (독립)
```

## Shared Feature Ownership

| Shared area | Rust crate | Reused by |
| --- | --- | --- |
| UI framework | `tench-ui` | every app |
| UI automation nodes | `ui-automation-core` | tench-ui, tench-ui-test |
| Headless test harness | `tench-ui-test` | every app (dev) |
| Local files/permissions | `fs-core` | media-core |
| Local storage policy | `storage-core` | view, image-runtime, media-runtime |
| Shared type definitions | `shared-types` | view, player, media-runtime |
| Media discovery | `media-core` | image-core, media-runtime |
| Image primitives | `image-core` | view, image-runtime |
| Image runtime | `image-runtime` | view, pixel-design |
| Pixel operations | `pixel-core` | pixel-design, image-runtime |
| Media runtime | `media-runtime` | player, composer |
| Media playback | `media-playback` | player |
| Subtitle processing | `subtitle-core` | media-runtime |
| Composer domain | `composer-core` | composer |

## Product Shell Rule

Product apps should only own product-specific composition and domain glue. If a
feature appears in multiple plan directories, it starts in a shared crate.

## Plan Mapping

| Plans | App slot | Primary shared crates |
| --- | --- | --- |
| `view` | `apps/view` | `image-core`, `media-core`, `image-runtime`, `storage-core`, `shared-types` |
| `pixel-design` | `apps/pixel-design` | `image-core`, `image-runtime`, `pixel-core` |
| `player` | `apps/player` | `media-core`, `media-runtime`, `media-playback`, `shared-types` |
| `composer` | `apps/composer` | `media-core`, `media-runtime`, `composer-core` |
