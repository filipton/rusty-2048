use anyhow::Result;
use board::{BoardData, Direction};
use crossterm::{
    event::{self, Event, KeyCode},
    execute, terminal,
};
use utils::clear_console;

mod board;
mod tests;
mod utils;

enum KeycodeResult {
    Direction(Direction),
    Quit,
    Continue,
}

pub struct CleanUp;
impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Unable to disable raw mode");
        execute!(std::io::stdout(), terminal::LeaveAlternateScreen)
            .expect("Unable to leave alternate screen");
    }
}

fn main() -> Result<()> {
    let _clean_up = CleanUp;
    terminal::enable_raw_mode()?;
    execute!(std::io::stdout(), terminal::EnterAlternateScreen)?;

    let mut board = BoardData::new();

    loop {
        match board.state {
            board::GameStatus::Lost => break,
            _ => (),
        }

        clear_console();
        board.print_board();

        if let Event::Key(event) = event::read().expect("Error") {
            // menu selector logic
            let direction = match event.code {
                KeyCode::Up | KeyCode::Char('k') => KeycodeResult::Direction(Direction::Up),
                KeyCode::Down | KeyCode::Char('j') => KeycodeResult::Direction(Direction::Down),
                KeyCode::Left | KeyCode::Char('h') => KeycodeResult::Direction(Direction::Left),
                KeyCode::Right | KeyCode::Char('l') => KeycodeResult::Direction(Direction::Right),
                KeyCode::Char('q') => KeycodeResult::Quit,
                KeyCode::Char('c') => {
                    if event.modifiers.contains(event::KeyModifiers::CONTROL) {
                        KeycodeResult::Quit
                    } else {
                        KeycodeResult::Continue
                    }
                }
                _ => KeycodeResult::Continue,
            };

            match direction {
                KeycodeResult::Direction(dir) => {
                    board.do_move(dir, false);
                }
                KeycodeResult::Quit => break,
                KeycodeResult::Continue => (),
            }
        }
    }

    drop(_clean_up);
    Ok(())
}
