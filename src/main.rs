use ask::{config::Config, history::History, openai::client::OpenAI, shell::Guesser};
use cliclack::select;
use std::{env, process::Command};

fn main() -> anyhow::Result<()> {
    let config = Config::get_or_default();

    let shell = config
        .shell
        .force_use
        .unwrap_or(Guesser::guess(config.shell.fallback));

    let question = env::args()
        .skip(1)
        .collect::<Vec<String>>()
        .join(" ")
        .trim()
        .to_string();

    if question.is_empty() {
        println!("Provide a question, e.g. \"ask list all large files in this folder\"");
        return Ok(());
    }

    let history = History::new(&shell)?;
    let ans_from_history = history.look_for_answer(&question);

    let answers = if config.command.enable_history && !ans_from_history.is_empty() {
        ans_from_history
    } else {
        if let Ok(answers) = OpenAI.ask(&question, &shell) {
            if config.command.enable_history {
                history.save_answer(&question, &answers)?;
            }
            answers
        } else {
            vec![]
        }
    };

    if answers.is_empty() {
        return Err(anyhow::Error::msg(
            "Could not find an answer to your question. Please try rewording it",
        ));
    }

    let options = answers
        .iter()
        .map(|option| (option, option, option))
        .collect::<Vec<_>>();

    let command = select("Select a command to run")
        .items(&options)
        .filter_mode()
        .interact()?
        .trim_start_matches('`')
        .trim_end_matches('`');

    let output = Command::new(shell).arg("-c").arg(&command).output()?;
    let formatted_output = String::from_utf8(if output.status.success() {
        output.stdout
    } else {
        output.stderr
    })?;

    if output.status.success() {
        println!("{formatted_output}");
    } else {
        eprintln!("{formatted_output}");
    }

    Ok(())
}
