use ask::{ai::client::AI, config::Config, shell::Guesser};
use crossterm::{
    cursor::MoveTo,
    event::{KeyCode, KeyEvent},
    style::Print,
    terminal::Clear,
};

#[tokio::main]
#[allow(clippy::cast_possible_truncation, clippy::too_many_lines)]
async fn main() -> anyhow::Result<()> {
    let config = Config::get_or_default();

    let shell = config
        .shell
        .force_use
        .unwrap_or(Guesser::guess(config.shell.fallback));

    let question = std::env::args()
        .skip(1)
        .collect::<Vec<String>>()
        .join(" ")
        .trim()
        .to_string();

    if question.is_empty() {
        println!("Provide a question, e.g. \"ask list all large files in this folder\"");
        return Ok(());
    }

    let rx = AI::ask(&question, &shell);
    let mut commands = vec![String::new(); config.command.choice_count as usize];

    println!("↕ + ↩ to select, (q) to quit or (e) to edit");

    crossterm::terminal::enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    let start_pos = crossterm::cursor::position()?;
    let (mut current, mut selected): (usize, Option<usize>) = (0, None);

    crossterm::execute!(stdout, crossterm::cursor::SavePosition)?;
    while let Ok((id, token)) = rx.recv() {
        commands[id as usize] = format!("{}{token}", commands[id as usize]);
        for _ in &commands {
            crossterm::execute!(
                &stdout,
                MoveTo(0, start_pos.1 + u16::from(id)),
                Print(&commands[id as usize]),
            )?;
        }
    }

    loop {
        if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
            match key {
                KeyEvent {
                    code: KeyCode::Up, ..
                } => {
                    current = current.saturating_sub(1);
                    crossterm::execute!(
                        stdout,
                        MoveTo(0, start_pos.1 + current as u16),
                        Print(&commands[current]),
                        Clear(crossterm::terminal::ClearType::UntilNewLine)
                    )?;
                }
                KeyEvent {
                    code: KeyCode::Down,
                    ..
                } => {
                    current = (current + 1).min(commands.len() - 1);
                    crossterm::execute!(
                        stdout,
                        MoveTo(0, start_pos.1 + current as u16),
                        Print(&commands[current]),
                        Clear(crossterm::terminal::ClearType::UntilNewLine)
                    )?;
                }
                KeyEvent {
                    code: KeyCode::Enter,
                    ..
                } => {
                    selected = Some(current);
                    break;
                }
                KeyEvent {
                    code: KeyCode::Char('e'),
                    ..
                } => {
                    std::fs::write(".command", &commands[current])?;
                    std::env::set_current_dir(".").unwrap();
                    let editor = std::env::var("EDITOR").unwrap_or(String::from("nano"));
                    let status = std::process::Command::new(&editor)
                        .arg(".command")
                        .status()?;
                    if status.success() {
                        let edited = std::fs::read_to_string(".command")?;
                        commands[current] = edited.trim().to_string();
                        crossterm::execute!(
                            stdout,
                            MoveTo(0, start_pos.1 + current as u16),
                            Print(&commands[current]),
                            Clear(crossterm::terminal::ClearType::UntilNewLine)
                        )?;
                        crossterm::terminal::enable_raw_mode()?;
                    }
                    std::fs::remove_file(".command")?;
                }
                KeyEvent {
                    code: KeyCode::Esc | KeyCode::Char('q'),
                    ..
                } => {
                    break;
                }
                _ => {}
            }
        }
    }

    crossterm::execute!(
        stdout,
        crossterm::cursor::MoveTo(0, start_pos.1 + commands.len() as u16 + 1)
    )?;

    crossterm::terminal::disable_raw_mode()?;

    if let Some(selected) = selected {
        crossterm::execute!(stdout, MoveTo(0, start_pos.1 + commands.len() as u16 + 1))?;

        let output = std::process::Command::new(&shell)
            .arg("-c")
            .arg(&commands[selected])
            .output()?;
        let formatted_output = String::from_utf8(if output.status.success() {
            output.stdout
        } else {
            output.stderr
        })?;

        if output.status.success() {
            println!("{formatted_output}");
        } else {
            eprintln!("{formatted_output}");
        }
    }

    Ok(())
}
