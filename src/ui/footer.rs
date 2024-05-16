use crossterm::{
    cursor,
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use std::io::{self, Write};

pub fn draw_footer(stdout: &mut io::Stdout, start_x: u16, rows: u16, container_width: u16) -> crossterm::Result<()> {
    // Draw light gray bar within the container width
    let bar = "─".repeat(container_width as usize);
    execute!(
        stdout,
        cursor::MoveTo(start_x, rows - 2),
        SetForegroundColor(Color::Grey),
        Print(&bar),
        ResetColor
    )?;

    // Centered footer options
    let footer_text = "+/- qty   c cart   q quit";
    let text_width = footer_text.len() as u16;
    let start_x_centered = start_x + (container_width.saturating_sub(text_width)) / 2;

    // Draw footer options in white
    execute!(
        stdout,
        cursor::MoveTo(start_x_centered, rows - 1),
        SetForegroundColor(Color::DarkGrey),
        Print(footer_text),
        ResetColor
    )?;

    execute!(stdout, SetForegroundColor(Color::Reset))
}
