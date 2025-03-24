use std::sync::Mutex;

use crate::discord_script::interpreter::Interpreter;
use crate::discord_script::parser::Parser;
use crate::discord_script::tokenizer::*;
use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::model::application::{CommandOptionType, ResolvedOption, ResolvedValue};

//Ideas:
//1) 2 versions of a script 1 that executes on an event(EventScript) and 1 that executes on
//   demand(Script)
//That means 2 runtimes one that is embedded into a discord-bot and one that is created on demand

pub fn run(options: &[ResolvedOption]) -> String {
    let mut res_string = String::new();
    let mut script = String::new();
    if let Some(ResolvedOption {
        value: ResolvedValue::String(file),
        ..
    }) = options.first()
    {
        //res_string = format!("Script file contents: {}", file);
        script = file.to_string();
    }

    let script_output = Mutex::new(res_string.clone());
    if !script.is_empty() {
        let tokens = Lexer::tokenize(script).unwrap();
        //let token_str = tokens
        //    .iter()
        //    .map(|x| format!("{x}"))
        //    .collect::<Vec<String>>()
        //    .join("\n");
        //res_string = format!("Tokens: {token_str}");

        let ast = Parser::parse(tokens);
        //res_string += format!("\n{ast:#?}").as_str();
        let mut inter = Interpreter::new();
        inter.null_expression_out = Some(Box::new(|value| {
            let mut val = script_output.lock().unwrap();
            *val += format!("\n {:?}", value).as_str();
            println!("\n {:?}", value);
        }));
        inter.on_error = Some(Box::new(|err| {
            let mut val = script_output.lock().unwrap();
            *val += format!("\n Error occured while evaluating an expression! {}", err).as_str();
        }));
        _ = inter.execute(*ast.unwrap());
    }
    let val = script_output.lock().unwrap();
    res_string += val.as_str();

    return res_string;
}

pub fn register() -> CreateCommand {
    CreateCommand::new("execute")
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
