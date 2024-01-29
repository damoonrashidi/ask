#[derive(Debug)]
pub struct Guesser;

impl Guesser {
    /// Will attempt to guess the name of the current shell
    /// # Errors
    /// This function will return an error if the shell could not reliably be found
    /// Or if the environment variables could not be read.
    pub fn guess() -> anyhow::Result<String> {
        if let Ok(true) = std::env::var("_").map(|value| value.contains("bin/nu")) {
            return Ok(String::from("nushell"));
        }

        if std::env::var("FISH_VERSION").is_ok() {
            return Ok(String::from("fish shell"));
        }

        let shell_env = std::env::var("SHELL")?;
        let shell = shell_env.split('/').last().unwrap_or("bash");

        Ok(shell.to_string())
    }
}
