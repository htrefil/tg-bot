#![feature(process_exitcode_placeholder)]
#![feature(async_closure)]
mod model;
mod words;

use futures::StreamExt;
use model::{Generator, Model};
use rand::rngs::OsRng;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;
use structopt::StructOpt;
use telegram_bot::prelude::*;
use telegram_bot::{Api, Error, UpdateKind};
use words::Words;

#[derive(StructOpt)]
struct Args {
    model: PathBuf,
    token: String,
}

async fn run(model: Model<String>, token: String) -> Result<(), Error> {
    let api = Api::new(token);
    let mut stream = api.stream();

    while let Some(update) = stream.next().await {
        let update = update?;
        match update.kind {
            UpdateKind::Message(message) => {
                let result = Generator::new(&model, &mut OsRng)
                    .map(|word| {
                        if word.chars().any(char::is_alphanumeric) {
                            format!(" {}", word)
                        } else {
                            format!("{}", word)
                        }
                    })
                    .take(10)
                    .collect::<String>();

                api.send(message.chat.text(result)).await?;
            }
            _ => {}
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> ExitCode {
    let args = match Args::from_iter_safe(env::args()) {
        Ok(args) => args,
        Err(err) => {
            println!("{}", err);
            return ExitCode::FAILURE;
        }
    };

    let data = match fs::read_to_string(&args.model) {
        Ok(data) => data,
        Err(err) => {
            println!("Error reading {}: {}", args.model.to_string_lossy(), err);
            return ExitCode::FAILURE;
        }
    };

    let words = Words::new(&data)
        .filter(|word| *word != "\n")
        .map(|word| word.to_string());
    let model = Model::new(words);

    if let Err(err) = run(model, args.token).await {
        println!("Error: {}", err);
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
