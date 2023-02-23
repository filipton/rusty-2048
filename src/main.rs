use anyhow::Result;
use board::BoardData;
use crossterm::{
    event::{self, Event, KeyCode},
    execute, terminal,
};
use utils::clear_console;

mod board;
mod tests;
mod utils;

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
        if board.lost {
            break;
        }

        clear_console();
        board.print_board();

        if let Event::Key(event) = event::read().expect("Error") {
            // menu selector logic
            match event.code {
                KeyCode::Up | KeyCode::Char('k') => {
                    board.move_up();
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    board.move_down();
                }
                KeyCode::Left | KeyCode::Char('h') => {
                    board.move_left();
                }
                KeyCode::Right | KeyCode::Char('l') => {
                    board.move_right();
                }
                KeyCode::Char('q') => break,
                KeyCode::Char('c') => {
                    if event.modifiers.contains(event::KeyModifiers::CONTROL) {
                        break;
                    }
                }
                _ => continue,
            }
        }
    }

    drop(_clean_up);
    Ok(())
}
