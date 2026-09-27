CREATE TABLE logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    status TEXT NOT NULL,
    activity TEXT NOT NULL,
    user_id BIGINT NOT NULL,
    unix_time BIGINT NOT NULL
);
