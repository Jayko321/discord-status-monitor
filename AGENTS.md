# Agent guide

## Project

Single Rust Discord bot (`Cargo.toml`). `src/main.rs` owns startup and Serenity events. Presence events are written through `src/storage.rs` to the SQLite `logs` table declared in `src/schema.rs`. `src/commands/` reads that table for `/check`, `/filter`, and `/whoplayed`, returning ephemeral replies. See `PROJECT_AUDIT.md` for the full data paths and current issues.

## Work here

- Read the affected event handler, storage function, and all command callers before changing data behavior. Keep fixes at the shared boundary when possible.
- Run `cargo check` for source changes (`cargo check --locked` when a lockfile exists). Add one focused runnable check for nontrivial logic. Do not claim gateway, live SQLite deployment, or Discord UI behavior from a compile check.
- Never print `DISCORD_TOKEN` or `.env` contents. Keep `.env` and local database files out of commits.
- `src/schema.rs` does not create the database. Add and track a real migration for schema changes; the current `.gitignore` excludes `/migrations`.
- Preserve unrelated work, including untracked `.serena/` files. Do not launch the bot or modify a live database just to verify a source edit.
- For library, SDK, API, or CLI documentation questions, use `ctx7`: resolve with `npx ctx7@latest library <name> "<topic>"`, then fetch `npx ctx7@latest docs <libraryId> "<topic>"`. Use a current version ID when needed.
