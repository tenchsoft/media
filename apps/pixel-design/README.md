# Pixel Design

Product app slot for layered image editing, canvas tools, and AI image workflows.

Primary plan source: `~/docs/plans/pixel-design`.

Expected shared foundations:

- `packages/app-shell`
- `packages/engine-client`
- `crates/image-core`
- `crates/media-core`
- `crates/job-core`
- `crates/plugin-core`

Reusable media loading, metadata, thumbnails, background jobs, and Engine calls
should stay outside this product shell.
