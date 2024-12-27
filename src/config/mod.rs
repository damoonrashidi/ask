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
    /// If enabled ask will store all answers in shell specific
    /// caches. If the same question is asked again the cache is
    /// checked first, to avoid going an extra HTTP call.
    ///
    /// Default: true
    ///
    /// Example: enable_history: true
    #[serde(default = "enable_history")]
    pub enable_history: bool,

    /// Which AI model to query. The model needs to be installed via Ollama.
    ///
    /// Default: "llama3.2:3b"
    ///
    /// Example: "mistal-nemo"
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
            model: "llama3.2:3b".to_owned(),
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
    String::from("llama3.2:3b")
}

fn choice_count() -> u8 {
    2
}

fn enable_history() -> bool {
    true
}

fn fallback_shell() -> String {
    String::from("/bin/bash")
}

fn force_shell() -> Option<String> {
    None
}
