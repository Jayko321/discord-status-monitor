use super::*;
use diesel::connection::SimpleConnection;

#[test]
fn saves_only_changed_presence_per_user() {
    use crate::schema::presences::dsl::*;

    let mut conn = SqliteConnection::establish(":memory:").unwrap();
    conn.batch_execute(include_str!(
        "../../migrations/20260927000000_create_logs/up.sql"
    ))
    .unwrap();

    save_presence_on(
        &mut conn,
        7,
        "online",
        vec!["Music".into(), "Game".into(), "Game".into()],
        123,
    )
    .unwrap();
    save_presence_on(
        &mut conn,
        7,
        "online",
        vec!["Game".into(), "Music".into()],
        124,
    )
    .unwrap();
    let rows = presences.load::<PresenceSnapshot>(&mut conn).unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].unix_time, 123);
    assert_eq!(rows[0].activity_names().unwrap(), ["Game", "Music"]);

    save_presence_on(
        &mut conn,
        7,
        "idle",
        vec!["Music".into(), "Game".into()],
        125,
    )
    .unwrap();
    let row = presences
        .find(7_i64)
        .first::<PresenceSnapshot>(&mut conn)
        .unwrap();
    assert_eq!(row.status, "idle");
    assert_eq!(row.activity_names().unwrap(), ["Game", "Music"]);
    assert_eq!(row.unix_time, 125);

    save_presence_on(&mut conn, 7, "idle", vec!["Game".into()], 126).unwrap();
    let row = presences
        .find(7_i64)
        .first::<PresenceSnapshot>(&mut conn)
        .unwrap();
    assert_eq!(row.activity_names().unwrap(), ["Game"]);
    assert_eq!(row.unix_time, 126);

    save_presence_on(&mut conn, 7, "idle", vec![], 127).unwrap();
    save_presence_on(&mut conn, 8, "online", vec!["Game".into()], 128).unwrap();
    let rows = presences.load::<PresenceSnapshot>(&mut conn).unwrap();
    assert_eq!(rows.len(), 2);
    let row = presences
        .find(7_i64)
        .first::<PresenceSnapshot>(&mut conn)
        .unwrap();
    assert!(row.activity_names().unwrap().is_empty());
}
