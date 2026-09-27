use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::model::application::{CommandOptionType, ResolvedOption, ResolvedValue};

use crate::storage::get_presence;

pub fn run(options: &[ResolvedOption]) -> String {
    let Some(ResolvedOption {
        value: ResolvedValue::User(user, _),
        ..
    }) = options.first()
    else {
        return "Please provide a valid user".to_string();
    };
    let limit = match options.get(1) {
        Some(ResolvedOption {
            value: ResolvedValue::Integer(value),
            ..
        }) => (*value).max(1) as usize,
        _ => 1,
    };

    let snapshot = match get_presence(user.id.into()) {
        Ok(Some(snapshot)) => snapshot,
        Ok(None) => return "Nothing was recorded in a database".to_string(),
        Err(err) => return err,
    };
    let mut activities = match snapshot.activity_names() {
        Ok(activities) => activities,
        Err(err) => return err,
    };
    if activities.is_empty() {
        activities.push(String::new());
    }
    activities
        .into_iter()
        .take(limit)
        .map(|activity| {
            format!(
                "Status: {}    Activity: {}   Time: <t:{}:R>",
                snapshot.status, activity, snapshot.unix_time
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn register() -> CreateCommand {
    CreateCommand::new("check")
        .description("Check a user's last observed status and activities")
        .add_option(
            CreateCommandOption::new(CommandOptionType::User, "id", "The user to lookup")
                .required(true),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Integer,
                "limit",
                "Maximum activities to show",
            )
            .min_int_value(1),
        )
}
