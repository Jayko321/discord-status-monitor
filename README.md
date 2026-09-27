# Discord Status Monitor

A Rust Discord bot that monitors and logs member presence/status changes in a guild. Built with [Serenity](https://github.com/serenity-rs/serenity) and [Diesel](https://diesel.rs/) (SQLite).

When a member's status (online/idle/dnd/offline) or activity (game/app name) changes, the bot records the event to a local SQLite database. Slash commands let you query the logged data.

## Commands

| Command | Description |
|---|---|
| `/check <user> [limit]` | Show a user's most recent status/activity logs |
| `/filter <user> <activity> [limit]` | Show a user's logs filtered by activity name |
| `/whoplayed <activity> [limit]` | List distinct users who played a specific activity |

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

The `logs` table has the following schema:

| Column | Type | Description |
|---|---|---|
| `id` | Integer | Primary key |
| `user_id` | BigInt | Discord user ID |
| `status` | Text | Status (online, idle, dnd, offline) |
| `activity` | Text | Activity / game name |
| `unix_time` | BigInt | Unix timestamp of the event |

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
