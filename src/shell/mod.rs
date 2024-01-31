#[derive(Debug)]
pub struct Guesser;

impl Guesser {
    /// Will attempt to guess the name of the current shell
    #[must_use]
    pub fn guess(fallback: String) -> String {
        if let Ok(true) = std::env::var("_").map(|value| value.contains("bin/nu")) {
            return String::from("nushell");
        }

        if std::env::var("$FISH_VERSION").is_ok() || std::env::var("FISH_VERSION").is_ok() {
            return String::from("fish");
        }

        return std::env::var("SHELL")
            .map(|shell| {
                shell
                    .clone()
                    .split('/')
                    .last()
                    .map_or(fallback, String::from)
                    .to_string()
            })
            .unwrap_or(String::from("bash"));
    }
}
