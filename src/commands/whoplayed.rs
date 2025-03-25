use diesel::prelude::*;
use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::model::application::{CommandOptionType, ResolvedOption, ResolvedValue};

use crate::storage::establish_connection;

pub fn run(options: &[ResolvedOption]) -> String {
    let mut res_string = String::new();
    let mut log_limit: Option<i64> = None;
    let mut activity_name: String = String::new();
    if let Some(ResolvedOption {
        value: ResolvedValue::Integer(limit),
        ..
    }) = options.get(1)
    {
        log_limit = Some(limit.clone());
    }
    if let Some(ResolvedOption {
        value: ResolvedValue::String(_activity),
        ..
    }) = options.get(0)
    {
        activity_name = String::from(*_activity);
    }

    use crate::schema::logs::dsl::*;
    match &mut establish_connection() {
        Ok(conn) => {
            let limit = log_limit.unwrap_or(1) as i64;
            let results = logs
                .filter(activity.eq(activity_name))
                .limit(limit)
                .select(user_id)
                .distinct()
                .order(id.desc())
                .load::<i64>(conn);

            match results {
                Ok(records) => {
                    res_string.clear();

                    res_string = records
                        .iter()
                        .map(|x| format!("<@{}>", x.to_string()))
                        .collect::<Vec<String>>()
                        .join("\n");
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
    //
    CreateCommand::new("whoplayed")
        .description("Check who played what")
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "activity", "Activity type")
                .required(true),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Integer,
                "limit",
                "How much data to fetch",
            )
            .min_int_value(1)
            .max_int_value(50),
        )
}
