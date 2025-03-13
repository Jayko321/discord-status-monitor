use crate::discord_script::tokenizer::*;
use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::model::application::{CommandOptionType, ResolvedOption, ResolvedValue};

#[allow(dead_code)]
struct EventScriptRuntime; //pub fn event(event: Event) -> Result<(), RuntimeError>
#[allow(dead_code)]
struct Runtime; // pub fn execute() -> Result<i64, RuntimeError>

//file.ds event script
//when(msg = message("cool")) { reply(msg, "response")}
//1. Read a file
//2. Try to find first token
//3. Convert a token to a keyword
//4. Proccess a keyword to ensure that it is correct
//5. Convert keyword to instruction
//6. goto 2

//Script
//when(msg = message("smart message")) { reply(msg, "smart_reply") }
//Global Script or a Script file
//pub function is_cool_name(name: string) -> bool {
//   return true;
//}
// //utils.ds if Script file
//Usage in a script
//import utils.ds //only if Script file
//when(message(match(""))) {
// if utils::is_cool_name() {
//   reply("Wow, what a cool name")
// }
//}

//Ideas:
//1) 2 versions of a script 1 that executes on an event(EventScript) and 1 that executes on
//   demand(Script)
//That means 2 runtimes one that is embedded into a discord-bot and one that is created on demand
//

//
//

//let token = Tokenizer.next_token();
//let keyword = from_token(token)
//if keyword.requires_token() {
// recurse until no tokens required
//}
//return keyword

//#[allow(dead_code)]
//enum KeyWordSpec {
////What keyword relies on
//When(Action),
//If(Condition),
//ElseIf(Condition),
//}

pub fn run(options: &[ResolvedOption]) -> String {
    let mut res_string = String::new();
    let mut script = String::new();
    if let Some(ResolvedOption {
        value: ResolvedValue::String(file),
        ..
    }) = options.first()
    {
        res_string = format!("Script file contents: {}", file);
        script = file.to_string();
    }

    if !script.is_empty() {
        let tokens = Lexer::tokenize(script);
        if let Ok(tokens) = tokens {
            let token_str = tokens.iter().map(|x| format!("{x}")).collect::<Vec<String>>().join("\n");
            res_string = format!("Tokens: {token_str}");
        } else {
            res_string = format!("Error: {:?}", tokens);
        }
    }

    return res_string;
}

pub fn register() -> CreateCommand {
    CreateCommand::new("add_event_script")
        .description("DEBUG ONLY Don't use")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "file",
                "DiscordScript file string",
            )
            .required(true),
        )
}
