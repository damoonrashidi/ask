use ask::{openai::client::OpenAI, shell::Guesser};
use std::env;

fn main() -> anyhow::Result<()> {
    let openai_key = env::var("OPENAI_KEY").expect("No OPENAI_KEY found in environment variables");

    let shell = Guesser::guess()?;
    let client = OpenAI::new(openai_key);

    let question = env::args().skip(1).collect::<Vec<String>>().join(" ");

    let Ok(answers) = client.ask(&question, &shell) else {
        println!("Could not reach OpenAI");
        return Ok(());
    };

    for answer in answers {
        println!("{answer}");
    }

    Ok(())
}
