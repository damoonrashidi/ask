use std::{fs::create_dir_all, io::Write, path::PathBuf};

#[derive(Debug)]
pub struct History<'s> {
    config_path: PathBuf,
    shell: &'s str,
}

impl<'a> History<'a> {
    /// Returns the new of this [`History`].
    ///
    /// # Errors
    ///
    /// This function will return an error if the users home directory could not be found
    /// or if the config directory could not be written to. Or if the users history file
    /// could not be created.
    pub fn new(shell: &'a str) -> anyhow::Result<Self> {
        let Some(config_path) =
            simple_home_dir::home_dir().map(|dir| dir.join(".config").join("ask"))
        else {
            return Err(anyhow::Error::msg("could not get history"));
        };

        History::make_history_file(&config_path, shell)?;

        Ok(Self { config_path, shell })
    }

    #[must_use]
    pub fn look_for_answer(&self, question: &String) -> Option<String> {
        let file_path = self.config_path.join(format!("{}.txt", self.shell));
        let Ok(line) = std::fs::read_to_string(file_path) else {
            return None;
        };
        let line = line
            .lines()
            .find(|line| line.starts_with(format!("{question}::::").as_str()))?;

        if let Some((_, answer)) = line.split_once("::::") {
            return Some(answer.to_string());
        }
        None
    }

    /// `save_answer`
    /// saves a found answer to disk for faster lookup
    /// # Errors
    ///
    /// This function will return an error if the answer could not be written to the history file
    pub fn save_answer(&self, question: &String, answer: &String) -> anyhow::Result<()> {
        let mut file = std::fs::OpenOptions::new()
            .append(true)
            .open(self.config_path.join(format!("{}.txt", self.shell)))?;

        writeln!(file, "{question}::::{answer}")?;
        Ok(())
    }

    fn make_history_file(config_path: &PathBuf, shell: &str) -> Result<(), std::io::Error> {
        create_dir_all(config_path)?;

        let history_file_path = config_path.join(format!("{shell}.txt"));
        if !history_file_path.exists() {
            std::fs::File::create(&history_file_path)?;
        }

        Ok(())
    }
}
