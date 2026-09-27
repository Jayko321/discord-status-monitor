CREATE TABLE presences (
    user_id INTEGER PRIMARY KEY,
    status TEXT NOT NULL,
    activities TEXT NOT NULL,
    unix_time INTEGER NOT NULL
);
