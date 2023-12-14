use std::env;

use anyhow::Result;
use chatgpt::config::ModelConfigurationBuilder;
use chatgpt::prelude::{ChatGPT, ChatGPTEngine};
use clap::{arg, ArgAction, command, Command, value_parser};

fn get_command() -> Command {
    command!()
        .arg_required_else_help(true)
        .subcommand(
            Command::new("ask")
                .arg_required_else_help(true)
                .about("Ask a single question").arg(
                arg!([question])
                    .help("The question to ask")
                    .required(true)
                    .action(ArgAction::Set)
                    .value_parser(value_parser!(String)),
            ),
        )
}

#[tokio::main]
async fn main() -> Result<()> {
    let matches = get_command().get_matches();
    let api_key = env::var("OPENAI_API_KEY")
        .unwrap_or_else(|_| panic!("{} not found in the environment variable", "OPENAI_API_KEY"));
    let gpt_client = ChatGPT::new_with_config(
        api_key,
        ModelConfigurationBuilder::default()
            .engine(ChatGPTEngine::Gpt4)
            .build()?,
    )?;
    println!("Using {}", gpt_client.config.engine);

    if let Some(question) = matches.subcommand_matches("question") {
        let q = question.get_one::<String>("question").unwrap().to_string();
        let resp = gpt_client.send_message(q).await?;
        println!("Response: {}", resp.message().content)
    }
    Ok(())
}
