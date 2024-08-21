use crossterm::{
    cursor,
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
};
use std::io::{self};

use crate::{products::get_products_lines, ui::handler::UIState};

impl UIState {
    pub fn show_store(&self, stdout: &mut io::Stdout, scroll_offset: u16) -> crossterm::Result<()> {
        execute!(stdout, cursor::MoveTo(0, self.start_y + self.header_height), Clear(ClearType::FromCursorDown))?;

        let mut y = self.start_y + self.header_height;
        let mut lines: Vec<(String, Color)> = get_products_lines(self);
        let available_height = terminal::size()?.1 - self.header_height - self.footer_height;

        // Calculate start and end lines based on scroll_offset
        let start_line = scroll_offset as usize;
        let end_line = start_line + available_height as usize;

        for (i, (line, color)) in lines.iter().enumerate().skip(start_line).take(end_line - start_line) {
            self.print_lines(stdout, &mut y, line, Some(*color))?;

            // Stop printing if we've filled the available space
            if y >= self.start_y + self.header_height + available_height {
                break;
            }
        }
        self.render_scrollbar_store(stdout, scroll_offset, lines.len() as u16, available_height)?;

        Ok(())
    }

    fn render_scrollbar_store(&self, stdout: &mut io::Stdout, scroll_offset: u16, content_height: u16, available_height: u16) -> crossterm::Result<()> {
        if content_height <= available_height {
            return Ok(()); // No need for a scrollbar if content fits within the available height
        }

        let scrollbar_height = ((available_height as f32 / content_height as f32) * available_height as f32).ceil() as u16;
        let scrollbar_position = ((scroll_offset as f32 / (content_height - available_height) as f32) * (available_height - scrollbar_height) as f32).ceil() as u16;

        for i in 0..available_height {
            let color = if i >= scrollbar_position && i < scrollbar_position + scrollbar_height {
                Color::Blue
            } else {
                Color::Reset
            };
            execute!(
                stdout,
                cursor::MoveTo(self.start_x + self.width + 1, self.start_y + self.header_height + i),
                SetForegroundColor(color),
                Print("▌"),
                ResetColor
            )?;
        }
        Ok(())
    }
}
