use ask::{history::History, openai::client::OpenAI, shell::Guesser};
use inquire::Select;
use std::{env, process::Command, string::ToString};

fn main() -> anyhow::Result<()> {
    let shell = Guesser::guess();

    let question = env::args()
        .skip(1)
        .collect::<Vec<String>>()
        .join(" ")
        .trim()
        .to_string();

    let history = History::new(&shell)?;

    if let Some(answer) = history.look_for_answer(&question) {
        let command = Select::new("Command suggestions", vec![answer]).prompt()?;
        let output = Command::new("sh").arg("-c").arg(&command).output()?;
        println!("{}", String::from_utf8(output.stdout)?);
    } else {
        let client = OpenAI::new(env::var("OPENAI_APIKEY").expect(
            "Could not find required \"OPENAI_APIKEY\" in environment variables. Make sure it's set.",
        ));
        if let Ok(answers) = client.ask(&question, &shell) {
            let command = Select::new("Command suggestions", answers).prompt()?;
            history.save_answer(&question, &command)?;
            let output = Command::new("sh").arg("-c").arg(&command).output()?;
            println!("{}", String::from_utf8(output.stdout)?);
        }
    }

    Ok(())
}
