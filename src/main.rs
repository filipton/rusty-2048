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
    let _clean_up = CleanUp;
    terminal::enable_raw_mode()?;
    execute!(std::io::stdout(), terminal::EnterAlternateScreen)?;

    let mut board = BoardData::new();

    loop {
        if board.lost {
            break;
        }

        clear_console();
        //board.print_board();

        let format = Format::new(7, 3);

        let colour_theme = vec![
            247, 78, 222, 220, 214, 208, 202, 196, 162, 160, 126, 90, 88, 54, 53, 52,
        ];
        let board_matrix = (0..16)
            .map(|x| {
                cell::Cell::new(
                    2_f64.powi(x + 1),
                    7,
                    *colour_theme.get(x as usize).unwrap() as u8,
                )
            })
            .collect::<Vec<_>>();

        let mut data = matrix::Matrix::new(4, board_matrix);
        let mut display = MatrixDisplay::new(&format, &mut data);
        //display.cell_at_cursor_position((1, 1)).color.bg = 226;
        display.print(&mut std::io::stdout(), &style::BordersStyle::None);

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
