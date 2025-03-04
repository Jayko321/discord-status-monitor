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
        let changed = presence.status.name();
        let activities = presence.activities;
        if let Some(guild) = presence.guild_id {
            if let Ok(member) = guild.member(_ctx.clone(), presence.user.id).await {
                if member.user.bot {
                    return;
                }

                let mut activity_str: String = "".to_string();
                for activity in activities {
                    println!("activities: {}", activity.name);
                    activity_str = activity.name;
                }
                println!("status: {}", changed);
                println!("user id: {}", presence.user.id);
                println!("{}", member.user.name);

                //Doesn't work, or rather works only for some not for everyone
                if let Some(nickname) = guild.member(_ctx, presence.user.id).await.unwrap().nick {
                    println!("server nickname: {}", nickname);
                }

                let unix_time = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as i64;
                new_log(NewLog {
                    user_id: presence.user.id.into(),
                    status: changed.to_string(),
                    activity: activity_str,
                    unix_time,
                })
                .unwrap_or_else(|err| {
                    println!("Error while inserting into a database: {}", err);
                });
            }
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            let content = match command.data.name.as_str() {
                "check" => Some(commands::check::run(&command.data.options())),
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
        let guild_id = GuildId::new(754762976371802203);

        _ = guild_id
            .set_commands(&ctx.http, vec![commands::check::register()])
            .await;
        // _ = Command::create_global_command(&ctx.http, commands::check::register()).await;
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("Not found");
    println!("{}", token);

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
