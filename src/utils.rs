use crossterm::{execute, terminal};

pub fn write(msg: String) {
    execute!(std::io::stdout(), crossterm::style::Print(msg)).expect("Error");
}

pub fn clear_console() {
    execute!(
        std::io::stdout(),
        crossterm::cursor::MoveTo(0, 0),
        terminal::Clear(terminal::ClearType::All)
    )
    .expect("Error");
}
