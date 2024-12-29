#[derive(Debug)]
pub struct Guesser;

impl Guesser {
    /// Will attempt to guess the name of the current shell
    /// It will fall back to the fallback shell if it can't
    /// guess the shell.
    #[must_use]
    pub fn guess(fallback: String) -> String {
        if let Ok(parent) = Guesser::get_parent_process_name() {
            return parent;
        }
        std::env::var("SHELL")
            .map(|shell| {
                shell
                    .split('/')
                    .last()
                    .map_or(fallback, String::from)
                    .to_string()
            })
            .unwrap_or(String::from("/bin/bash"))
    }

    fn get_parent_process_name() -> Result<String, std::io::Error> {
        #[cfg(target_os = "windows")]
        {
            return Ok("pwsh".to_string());
        }

        #[cfg(any(unix, target_os = "macos"))]
        {
            let child = std::process::Command::new("ps")
                .arg("-p")
                .arg(std::os::unix::process::parent_id().to_string())
                .arg("-o")
                .arg("comm=")
                .output()?;
            let parent_process_name = String::from_utf8_lossy(&child.stdout)
                .trim()
                .chars()
                .filter(char::is_ascii)
                .collect();
            Ok(parent_process_name)
        }
    }
}
