# Discord Status Monitor

A Rust Discord bot that stores each member's last observed presence in a guild. Built with [Serenity](https://github.com/serenity-rs/serenity) and [Diesel](https://diesel.rs/) (SQLite).

The bot stores one row per user in a local SQLite database. Repeated updates with the same status and activities are skipped. After a restart, saved states may be stale until Discord sends new presence updates.

## Commands

| Command | Description |
|---|---|
| `/check <user> [limit]` | Show a user's last observed status and up to `limit` activities (default 1) |
| `/filter <user> <activity> [limit]` | Show the user's last observed status if the activity matches; `limit` is retained but at most one result exists |
| `/whoplayed <activity> [limit]` | List users whose last observed activity set contains the name |

## Setup

### Prerequisites

- Rust (edition 2021)
- [Diesel CLI](https://diesel.rs/guides/getting-started) for SQLite

### Configuration

Create a `.env` file in the project root:

```env
DISCORD_TOKEN=your_bot_token
DATABASE_URL=db.sqlite
DISCORD_GUILD_ID=your_guild_id
DISCORD_ALLOWED_IDS=user_id_1,user_id_2,user_id_3
```

| Variable | Description |
|---|---|
| `DISCORD_TOKEN` | Discord bot token |
| `DATABASE_URL` | Path to the SQLite database file |
| `DISCORD_GUILD_ID` | ID of the guild to register slash commands on |
| `DISCORD_ALLOWED_IDS` | Comma-separated Discord user IDs allowed to use commands |

### Database Setup

```sh
diesel migration run
```

The `presences` table has the following schema:

| Column | Type | Description |
|---|---|---|
| `user_id` | Integer | Discord user ID and primary key |
| `status` | Text | Status (online, idle, dnd, offline) |
| `activities` | Text | JSON array of activity names |
| `unix_time` | Integer | Unix timestamp of the last observed change |

### Run

```sh
cargo run
```

The bot requires the `GUILD_PRESENCES` gateway intent enabled in the Discord Developer Portal.

## Stack

- **Runtime:** Tokio (multi-threaded)
- **Discord API:** Serenity 0.12
- **Database:** Diesel 2.2 + SQLite
- **Config:** dotenv
