use super::*;
use diesel::connection::SimpleConnection;

#[test]
fn only_current_players_obey_limit() {
    let mut conn = SqliteConnection::establish(":memory:").unwrap();
    conn.batch_execute(include_str!(
        "../../migrations/20260927000000_create_logs/up.sql"
    ))
    .unwrap();
    conn.batch_execute(
        "INSERT INTO presences (user_id, status, activities, unix_time) VALUES
        (1, 'online', '[\"Other\"]', 6), (2, 'online', '[\"Game\"]', 2),
        (3, 'online', '[\"Game\",\"Music\"]', 4), (4, 'online', '[\"Other\"]', 5)",
    )
    .unwrap();
    assert_eq!(recent_players(&mut conn, "Game", 2).unwrap(), [3, 2]);
    assert_eq!(recent_players(&mut conn, "Music", 2).unwrap(), [3]);
}
