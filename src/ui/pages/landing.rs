use crossterm::{
    cursor,
    execute,
    style::{Print, ResetColor},
    terminal::{Clear, ClearType},
};
use std::io::{self, Write};

use crate::ui::{handler::UIState, header::center_text};

impl UIState {
    fn center_multiline_text(&self, text: &str, width: u16) -> Vec<String> {
        text.lines()
            .map(|line| center_text(line, width as usize))
            .collect()
    }


    pub fn show_landing(&self, stdout: &mut io::Stdout) -> crossterm::Result<()> {
        execute!(stdout, cursor::MoveTo(0, self.start_y + self.header_height), Clear(ClearType::FromCursorDown))?;

        let landing_text = "Welcome to the shop!\n\nNavigate using [<-, ->] keys!\n";
        let centered_lines = self.center_multiline_text(landing_text, self.width);

        // Calculate the starting y position to center the text vertically
        let text_height = centered_lines.len() as u16;

        let mut y = self.start_y + self.header_height;
        for line in centered_lines {
            let start_x = self.start_x + (self.width / 2) - (line.len() as u16 / 2);
            execute!(stdout, cursor::MoveTo(start_x, y), Print(&line), ResetColor)?;
            y += 1;
        }

        Ok(())
    }
}


