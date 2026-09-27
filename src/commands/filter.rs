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
    let Some(ResolvedOption {
        value: ResolvedValue::String(activity_name),
        ..
    }) = options.get(1)
    else {
        return "Please provide a valid activity".to_string();
    };

    let snapshot = match get_presence(user.id.into()) {
        Ok(Some(snapshot)) => snapshot,
        Ok(None) => return "Nothing was recorded in a database".to_string(),
        Err(err) => return err,
    };
    match snapshot.activity_names() {
        Ok(activities) if activities.iter().any(|activity| activity == activity_name) => format!(
            "Status: {}    Activity: {}   Time: <t:{}:R>",
            snapshot.status, activity_name, snapshot.unix_time
        ),
        Ok(_) => "Nothing was recorded in a database".to_string(),
        Err(err) => err,
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("filter")
        .description("Check a user's last observed status for an activity")
        .add_option(
            CreateCommandOption::new(CommandOptionType::User, "id", "The user to lookup")
                .required(true),
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "activity", "Activity name")
                .required(true),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Integer,
                "limit",
                "Maximum matching activities to show (at most one)",
            )
            .min_int_value(1),
        )
}
