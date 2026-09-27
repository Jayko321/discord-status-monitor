use diesel::prelude::*;
use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::model::application::{CommandOptionType, ResolvedOption, ResolvedValue};

use crate::storage::{establish_connection, PresenceSnapshot};

fn recent_players(
    conn: &mut SqliteConnection,
    activity_name: &str,
    limit: i64,
) -> Result<Vec<i64>, String> {
    use crate::schema::presences::dsl::*;
    // ponytail: scans one row per user; normalize and index activities if this becomes slow.
    let snapshots = presences
        .order((unix_time.desc(), user_id.desc()))
        .select(PresenceSnapshot::as_select())
        .load::<PresenceSnapshot>(conn)
        .map_err(|err| err.to_string())?;
    let mut players = Vec::new();
    for snapshot in snapshots {
        if snapshot
            .activity_names()?
            .iter()
            .any(|activity| activity == activity_name)
        {
            players.push(snapshot.user_id);
            if players.len() >= limit.max(1) as usize {
                break;
            }
        }
    }
    Ok(players)
}

pub fn run(options: &[ResolvedOption]) -> String {
    let Some(ResolvedOption {
        value: ResolvedValue::String(activity_name),
        ..
    }) = options.first()
    else {
        return "Please provide a valid activity".to_string();
    };
    let limit = match options.get(1) {
        Some(ResolvedOption {
            value: ResolvedValue::Integer(value),
            ..
        }) => *value,
        _ => 1,
    };
    let mut conn = match establish_connection() {
        Ok(conn) => conn,
        Err(err) => return err,
    };
    let players = match recent_players(&mut conn, activity_name, limit) {
        Ok(players) => players,
        Err(err) => return err,
    };
    if players.is_empty() {
        return "Nothing was recorded in a database".to_string();
    }
    players
        .iter()
        .map(|user_id| format!("<@{}>", user_id))
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
#[path = "../../tests/unit/whoplayed.rs"]
mod tests;

pub fn register() -> CreateCommand {
    CreateCommand::new("whoplayed")
        .description("Check whose last observed activity matches")
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "activity", "Activity type")
                .required(true),
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::Integer, "limit", "Maximum users to show")
                .min_int_value(1)
                .max_int_value(50),
        )
}
