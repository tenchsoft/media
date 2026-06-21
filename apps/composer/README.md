# Composer

Product app slot for timeline editing, clips, rendering, subtitles, and AI video
workflows.

Primary plan source: `~/docs/plans/composer`.

Expected shared foundations:

- `packages/app-shell`
- `packages/engine-client`
- `crates/media-core`
- `crates/job-core`
- `crates/plugin-core`

Timeline and editing-domain models belong here. Media discovery, cache identity,
jobs, plugins, and Engine access should stay shared.
