use super::logs_for_activities;

#[test]
fn logs_each_activity_and_preserves_empty_presence() {
    let logs = logs_for_activities(7, "online", vec!["Game".into(), "Music".into()], 123);
    assert_eq!(logs.len(), 2);
    assert_eq!(
        logs.iter()
            .map(|log| log.activity.as_str())
            .collect::<Vec<_>>(),
        ["Game", "Music"]
    );
    assert!(logs
        .iter()
        .all(|log| log.user_id == 7 && log.status == "online" && log.unix_time == 123));

    let empty = logs_for_activities(7, "idle", vec![], 124);
    assert_eq!(empty.len(), 1);
    assert_eq!(empty[0].activity, "");
    assert_eq!(empty[0].status, "idle");
    assert_eq!(empty[0].unix_time, 124);
}
