use serde::Deserialize;
use std::{fs::File, io::Read};

#[derive(Deserialize, Debug, Default)]
pub struct Config {
    #[serde(default)]
    pub command: CommandConf,

    #[serde(default)]
    pub shell: ShellConf,
}

impl Config {
    /**
    Will try to read the config toml file from the users home directory.
    If the file could not be read or if the config could not be parsed
    the function will return a default config.
    */
    #[must_use]
    pub fn get_or_default() -> Self {
        let Some(config_path) = simple_home_dir::home_dir()
            .map(|dir| dir.join(".config").join("ask").join("config.toml"))
        else {
            return Config::default();
        };

        let Ok(mut file) = File::open(config_path) else {
            return Config::default();
        };

        let mut contents = String::new();
        let Ok(_) = file.read_to_string(&mut contents) else {
            return Config::default();
        };

        match toml::from_str::<Config>(&contents.to_string()) {
            Ok(config) => config,
            Err(err) => {
                println!("{err:?}");
                Config::default()
            }
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct CommandConf {
    /// If enabled will cache responses per shell and use the same answer
    /// if the same question is asked again.
    ///
    /// Example: enable_history: true
    #[serde(default = "enable_history")]
    pub enable_history: bool,

    /// Which ChatGPT model to query. A full list of models can be found in the
    /// [API docs](https://platform.openai.com/docs/models/overview).
    ///
    /// Default: "gpt-4-1106-preview"
    ///
    /// Example: variation_count: 3
    #[serde(default = "default_model")]
    pub model: String,

    /// The number of answer choices to return from ChatGPT.
    /// Default: 2, min: 1
    ///
    /// Example: variation_count: 3
    #[serde(default = "choice_count")]
    pub choice_count: u8,
}

impl Default for CommandConf {
    fn default() -> Self {
        Self {
            enable_history: true,
            choice_count: 2,
            model: "gpt-4-1106-preview".to_owned(),
        }
    }
}

#[derive(Deserialize, Debug, Default)]
pub struct ShellConf {
    /// If set, it will force `ask` to use the set shell and it will no longer try to guess.
    ///
    /// Default: None
    ///
    /// Example: force_shell: "fish"
    #[serde(default = "force_shell")]
    pub force_use: Option<String>,

    /// The shell to use if the guesser failed to reliably guess the current shell.
    ///
    /// Default: "bash"
    ///
    /// Example: fallback: "zsh"
    #[serde(default = "fallback_shell")]
    pub fallback: String,
}

fn default_model() -> String {
    String::from("gpt-4-1106-preview")
}

fn choice_count() -> u8 {
    2
}

fn enable_history() -> bool {
    true
}

fn fallback_shell() -> String {
    String::from("bash")
}

fn force_shell() -> Option<String> {
    None
}
