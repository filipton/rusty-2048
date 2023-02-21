use crossterm::{execute, terminal, style::Color};

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

pub fn print_formatted(text: String, foreground_color: Color, background_color: Color) {
    execute!(
        std::io::stdout(),
        crossterm::style::SetBackgroundColor(background_color),
        crossterm::style::SetForegroundColor(foreground_color),
        crossterm::style::Print(text),
        crossterm::style::SetBackgroundColor(Color::Reset),
        crossterm::style::SetForegroundColor(Color::Reset),
    )
    .expect("Error");
}
