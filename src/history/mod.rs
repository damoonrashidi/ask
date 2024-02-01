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

    /// If this question has previously been asked for the current shell this
    /// function will return a vec of answers. If the question has not been
    /// asked, if the history file could not be read if the question doesn't
    /// have any answers the function will return the empty vec.
    #[must_use]
    pub fn look_for_answer(&self, question: &String) -> Vec<String> {
        let file_path = self.config_path.join(format!("{}.txt", self.shell));
        let Ok(line) = std::fs::read_to_string(file_path) else {
            return vec![];
        };
        let Some(line) = line
            .lines()
            .find(|line| line.starts_with(format!("{question}::::").as_str()))
        else {
            return vec![];
        };

        line.split("::::").skip(1).map(String::from).collect()
    }

    /// `save_answer`
    /// saves a found answer to disk for faster lookup
    /// # Errors
    ///
    /// This function will return an error if the answer could not be written to the history file
    pub fn save_answer(&self, question: &String, answers: &[String]) -> anyhow::Result<()> {
        let mut file = std::fs::OpenOptions::new()
            .append(true)
            .open(self.config_path.join(format!("{}.txt", self.shell)))?;

        let delimitered_answers = answers.join("::::");
        writeln!(file, "{question}::::{delimitered_answers}")?;
        Ok(())
    }

    /// Creates a new history file (if one doesn't exist) for the given shell.
    fn make_history_file(config_path: &PathBuf, shell: &str) -> Result<(), std::io::Error> {
        create_dir_all(config_path)?;

        let history_file_path = config_path.join(format!("{shell}.txt"));
        if !history_file_path.exists() {
            std::fs::File::create(&history_file_path)?;
        }

        Ok(())
    }
}
