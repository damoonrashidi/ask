use ask::{history::History, openai::client::OpenAI, shell::Guesser};
use std::env;

fn main() -> anyhow::Result<()> {
    let openai_key = env::var("OPENAI_KEY").expect("No OPENAI_KEY found in environment variables");

    let shell = Guesser::guess();
    let client = OpenAI::new(openai_key);

    let question = env::args()
        .skip(1)
        .collect::<Vec<String>>()
        .join(" ")
        .trim()
        .to_string();

    let history = History::new(&shell)?;

    if let Some(answer) = history.look_for_answer(&question) {
        println!("{answer}");
        return Ok(());
    }

    if let Ok(answers) = client.ask(&question, &shell) {
        for answer in answers {
            let _ = history.save_answer(&question, &answer);
            println!("{answer}");
        }
    };

    Ok(())
}
