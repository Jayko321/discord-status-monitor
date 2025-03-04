use diesel::prelude::*;
use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::model::application::{CommandOptionType, ResolvedOption, ResolvedValue};

use crate::storage::{establish_connection, Log};

pub fn run(options: &[ResolvedOption]) -> String {
    let mut res_string;
    let mut _user_id: i64;
    let mut log_limit: Option<i64> = None;
    if let Some(ResolvedOption {
        value: ResolvedValue::User(user, _),
        ..
    }) = options.first()
    {
        res_string = format!("{}'s id is {}", user.tag(), user.id);
        _user_id = user.id.into();
    } else {
        return "Please provide a valid user".to_string();
    }
    if let Some(ResolvedOption {
        value: ResolvedValue::Integer(limit),
        ..
    }) = options.get(1)
    {
        res_string = format!(
            "{} \n limit option was found it is set to {}",
            res_string, limit
        );
        log_limit = Some(limit.clone());
    }

    use crate::schema::logs::dsl::*;
    match &mut establish_connection() {
        Ok(conn) => {
            let limit = log_limit.unwrap_or(1) as i64;
            let results = logs
                .filter(user_id.eq(_user_id))
                .limit(limit)
                .select(Log::as_select())
                .order(id.desc())
                .load(conn);

            match results {
                Ok(records) => {
                    res_string.clear();
                    for record in records {
                        res_string = format!(
                            "{}\nStatus: {}    Activity: {}   Time: <t:{}:R>   ",
                            res_string, record.status, record.activity, record.unix_time
                        );
                    }
                    if res_string.is_empty() {
                        res_string += "Nothing was recorded in a database";
                    }
                }
                Err(err) => return err.to_string(),
            }
        }
        Err(err) => return err.to_string(),
    }

    return res_string;
}

pub fn register() -> CreateCommand {
    CreateCommand::new("check")
        .description("Check most recent user activity")
        .add_option(
            CreateCommandOption::new(CommandOptionType::User, "id", "The user to lookup")
                .required(true),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Integer,
                "limit",
                "How much data to fetch",
            )
            .min_int_value(1),
        )
}
