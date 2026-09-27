#![cfg_attr(not(debug_assertions), deny(warnings))]
pub mod commands;
pub mod schema;
pub mod storage;

use dotenv::dotenv;
use serenity::all::CreateInteractionResponse;
use serenity::all::CreateInteractionResponseMessage;
use serenity::all::GuildId;
use serenity::all::Interaction;
use serenity::all::Presence;
use serenity::all::Ready;
use serenity::all::*;
use serenity::async_trait;

use std::env;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use self::storage::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn presence_update(&self, _ctx: Context, presence: Presence) {
        if presence.user.bot == Some(true) {
            return;
        }

        let unix_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        let logs = logs_for_activities(
            presence.user.id.into(),
            presence.status.name(),
            presence
                .activities
                .into_iter()
                .map(|activity| activity.name)
                .collect(),
            unix_time,
        );
        new_logs(&logs).unwrap_or_else(|err| {
            println!("Error while inserting into a database: {}", err);
        });
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            let allowed_ids: Vec<u64> = env::var("DISCORD_ALLOWED_IDS")
                .unwrap_or_default()
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            if !allowed_ids.contains(&command.user.id.into()) {
                return;
            }

            let content = match command.data.name.as_str() {
                "check" => Some(commands::check::run(&command.data.options())),
                "filter" => Some(commands::filter::run(&command.data.options())),
                "whoplayed" => Some(commands::whoplayed::run(&command.data.options())),
                _ => Some("No command".to_string()),
            };

            if let Some(content) = content {
                let data = CreateInteractionResponseMessage::new()
                    .content(content)
                    .ephemeral(true);
                let builder = CreateInteractionResponse::Message(data);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("Cannot respond to slash command: {why}");
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        let guild_id = GuildId::new(
            env::var("DISCORD_GUILD_ID")
                .expect("DISCORD_GUILD_ID must be set")
                .parse()
                .expect("DISCORD_GUILD_ID must be a valid integer"),
        );

        _ = guild_id
            .set_commands(
                &ctx.http,
                vec![
                    commands::check::register(),
                    commands::filter::register(),
                    commands::whoplayed::register(),
                ],
            )
            .await;
        // _ = Command::create_global_command(&ctx.http, commands::check::register()).await;
    }
}

fn logs_for_activities(
    user_id: i64,
    status: &str,
    activities: Vec<String>,
    unix_time: i64,
) -> Vec<NewLog> {
    let activities = if activities.is_empty() {
        vec![String::new()]
    } else {
        activities
    };
    activities
        .into_iter()
        .map(|activity| NewLog {
            user_id,
            status: status.to_string(),
            activity,
            unix_time,
        })
        .collect()
}

#[tokio::main]
async fn main() {
    //assert!(false, "TODO: write tests for a Lexer");
    dotenv().ok();
    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("Not found");

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_PRESENCES;

    // Create a new instance of the Client, logging in as a bot.
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}

#[cfg(test)]
#[path = "../tests/unit/presence.rs"]
mod presence_tests;
