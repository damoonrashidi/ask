use ask::{config::Config, history::History, openai::client::OpenAI, shell::Guesser};
use inquire::Select;
use std::{env, process::Command, string::ToString};

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

    let history = History::new(&shell)?;
    let ans_from_history = history.look_for_answer(&question);

    let answers = if config.command.enable_history && !ans_from_history.is_empty() {
        ans_from_history
    } else {
        let client = OpenAI::new(env::var("OPENAI_APIKEY").expect(
        "Could not find required \"OPENAI_APIKEY\" in environment variables. Make sure it's set.",
    ));
        if let Ok(answers) = client.ask(&question, &shell) {
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

    let Ok(command) = Select::new("Command suggestions", answers)
        .with_help_message("↕ to select, ↵  to run and ESC to cancel")
        .prompt()
    else {
        return Ok(());
    };

    let output = Command::new(shell).arg("-c").arg(&command).output()?;
    let formatted_output = String::from_utf8(if output.status.success() {
        output.stdout.clone()
    } else {
        output.stderr
    })?;

    println!("{formatted_output}");

    Ok(())
}
