use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> Result<SqliteConnection, String> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    match SqliteConnection::establish(&database_url) {
        Ok(conn) => return Ok(conn),
        Err(err) => return Err(err.to_string()),
    }
}

pub fn new_log(log: NewLog) -> Result<(), String> {
    use crate::schema::logs::dsl::*;
    match &mut establish_connection() {
        Ok(conn) => {
            let res = diesel::insert_into(logs).values(log).execute(conn);

            if let Some(err) = res.err() {
                return Err(err.to_string());
            }

            Ok(())
        }
        Err(err) => return Err(err.to_string()),
    }
}

pub fn get_log(_id: i32) -> Result<Log, String> {
    use crate::schema::logs::dsl::*;
    match &mut establish_connection() {
        Ok(conn) => {
            let results = logs
                .filter(id.eq(_id))
                .limit(1)
                .select(Log::as_select())
                .load(conn);

            match results {
                Ok(log) => {
                    if log.len() > 0 {
                        let res: Log = log[0].clone();
                        return Ok(res);
                    } else {
                        return Err("Not found".to_string());
                    }
                }
                Err(err) => return Err(err.to_string()),
            }
        }
        Err(err) => return Err(err.to_string()),
    }

    // Err("Unknown error".to_string())
}

#[derive(Queryable, Selectable, Clone, Debug)]
#[diesel(table_name = crate::schema::logs)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Log {
    pub id: i32,
    pub user_id: i64,
    pub status: String,
    pub activity: String,
    pub unix_time: i64,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::logs)]
pub struct NewLog {
    pub user_id: i64,
    pub status: String,
    pub activity: String,
    pub unix_time: i64,
}
