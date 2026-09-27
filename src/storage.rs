use diesel::prelude::*;
use diesel::sql_types::{BigInt, Text};
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> Result<SqliteConnection, String> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url).map_err(|err| err.to_string())
}

pub fn save_presence(
    user_id: i64,
    status: &str,
    activities: Vec<String>,
    unix_time: i64,
) -> Result<(), String> {
    save_presence_on(
        &mut establish_connection()?,
        user_id,
        status,
        activities,
        unix_time,
    )
}

fn save_presence_on(
    conn: &mut SqliteConnection,
    user_id: i64,
    status: &str,
    mut activities: Vec<String>,
    unix_time: i64,
) -> Result<(), String> {
    activities.sort_unstable();
    activities.dedup();
    let activities = serenity::json::to_string(&activities).map_err(|err| err.to_string())?;

    diesel::sql_query(
        "INSERT INTO presences (user_id, status, activities, unix_time)
         VALUES (?, ?, ?, ?)
         ON CONFLICT(user_id) DO UPDATE SET
             status = excluded.status,
             activities = excluded.activities,
             unix_time = excluded.unix_time
         WHERE presences.status <> excluded.status
            OR presences.activities <> excluded.activities",
    )
    .bind::<BigInt, _>(user_id)
    .bind::<Text, _>(status)
    .bind::<Text, _>(activities)
    .bind::<BigInt, _>(unix_time)
    .execute(conn)
    .map(|_| ())
    .map_err(|err| err.to_string())
}

pub fn get_presence(id: i64) -> Result<Option<PresenceSnapshot>, String> {
    use crate::schema::presences::dsl::*;
    presences
        .find(id)
        .select(PresenceSnapshot::as_select())
        .first(&mut establish_connection()?)
        .optional()
        .map_err(|err| err.to_string())
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::presences)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct PresenceSnapshot {
    pub user_id: i64,
    pub status: String,
    pub activities: String,
    pub unix_time: i64,
}

impl PresenceSnapshot {
    pub fn activity_names(&self) -> Result<Vec<String>, String> {
        serenity::json::from_str(&self.activities).map_err(|err| err.to_string())
    }
}

#[cfg(test)]
#[path = "../tests/unit/presence.rs"]
mod tests;
