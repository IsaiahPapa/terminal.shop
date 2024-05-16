use crossterm::{
    cursor,
    execute,
    style::Color,
    terminal::{Clear, ClearType},
};
use std::io::{self, Write};

use crate::ui::handler::UIState;

impl UIState {
    pub fn show_store(&self, stdout: &mut io::Stdout) -> crossterm::Result<()> {
        execute!(stdout, cursor::MoveTo(0, self.start_y + self.header_height), Clear(ClearType::FromCursorDown))?;

        let mut y = self.start_y + self.header_height;
        for product in &self.products {
            self.print_lines(stdout, &mut y, &product.name, Some(Color::White))?;
            y += 1; // Space between lines
            let attributes = product.attributes.join(" | ");
            self.print_lines(stdout, &mut y, &format!("{}", attributes), Some(Color::DarkGrey));
            y += 1;
            self.print_lines(stdout, &mut y, &format!("${:.2}", product.price), Some(Color::Rgb { r: 255, g: 90, b: 0 }))?;
            y += 1;
            self.print_lines(stdout, &mut y, &format!("{}", product.description), Some(Color::DarkGrey))?;
            y += 1;
            self.print_lines(stdout, &mut y, &format!("{}", if product.quantity > 0 {
                format!("{} Bags left", product.quantity)
            } else {
                "Sold out!".to_string()
            }), Some(Color::White))?;
            y += 2; // Add extra lines between products
        }
        Ok(())
    }
}
