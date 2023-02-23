use anyhow::Result;
use board::BoardData;
use crossterm::{
    event::{self, Event, KeyCode},
    execute, terminal,
};
use matrix_display::*;
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
    let format = Format::new(7, 3);
    let board = vec![
        2048, 4096, 0,
        16000, 2, 256
    ]
    .iter()
    .enumerate()
    .map(|(i, x)| {
        let ansi_fg = 33;
        let mut ansi_bg = 0;
        if i % 2 + (i / 8) % 2 == 1 {
            ansi_bg = 7;
        }
        cell::Cell::new(x.clone(), ansi_fg, ansi_bg)
    })
    .collect::<Vec<_>>();
    let mut data = matrix::Matrix::new(3, board);
    let mut display = MatrixDisplay::new(&format, &mut data);
    //display.cell_at_cursor_position((13, 6)).color.bg = 10;
    display.print(&mut std::io::stdout(), &style::BordersStyle::None);

    return Ok(());

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
