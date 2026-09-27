use super::*;
use diesel::connection::SimpleConnection;

#[test]
fn latest_distinct_players_obey_limit() {
    let mut conn = SqliteConnection::establish(":memory:").unwrap();
    conn.batch_execute(include_str!(
        "../../migrations/20260927000000_create_logs/up.sql"
    ))
    .unwrap();
    conn.batch_execute(
        "INSERT INTO logs (user_id, status, activity, unix_time) VALUES
        (1, 'online', 'Game', 1), (2, 'online', 'Game', 2),
        (1, 'online', 'Game', 3), (3, 'online', 'Game', 4),
        (4, 'online', 'Other', 5)",
    )
    .unwrap();
    assert_eq!(recent_players(&mut conn, "Game", 2).unwrap(), [3, 1]);
}
