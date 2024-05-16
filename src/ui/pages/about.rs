use crossterm::{
    cursor,
    execute,
    style::Color,
    terminal::{Clear, ClearType},
};
use std::io::{self, Write};

use crate::ui::handler::UIState;

impl UIState {
    pub fn show_about(&self, stdout: &mut io::Stdout) -> crossterm::Result<()> {
        execute!(stdout, cursor::MoveTo(0, self.start_y + self.header_height), Clear(ClearType::FromCursorDown))?;
        let mut y = self.start_y + self.header_height;
        self.print_lines(stdout, &mut y, "About SSH Store:\nWe provide the best products directly from the terminal.\n", Some(Color::White))
    }
}