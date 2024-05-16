use crate::products::Product;
use crossterm::{
    cursor,
    execute,
    style::{Color, Print, SetForegroundColor, ResetColor},
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use std::io::{self, Stdout, Write};
use textwrap::wrap;

pub enum Page {
    Store,
    About,
    Landing,
    FAQ
    // Add other pages as needed
}


pub struct UIState {
    pub header_height: u16,
    pub footer_height: u16,
    pub products: Vec<Product>,
    pub start_x: u16,
    pub start_y: u16,
    pub width: u16,
    pub current_page: Page,
    pub scroll_offset: u16
}

impl UIState {
    pub fn new(products: Vec<Product>, term_width: u16, term_height: u16) -> Self {
        let width = 60;
        let height = 45;
        let start_x = (term_width.saturating_sub(width)) / 2;
        let start_y = (term_height.saturating_sub(height)) / 2;

        UIState {
            header_height: 0,
            footer_height: 2, // Footer height is constant
            products,
            start_x,
            start_y,
            width,
            current_page: Page::Landing,
            scroll_offset: 0,
        }
    }

    pub fn update_dimensions(&mut self, term_width: u16, term_height: u16) {
        self.start_x = (term_width.saturating_sub(self.width)) / 2;
        self.start_y = (term_height.saturating_sub(45)) / 2;
        self.width = term_width.min(60);
    }

    pub fn print_lines(&self, stdout: &mut io::Stdout, start_y: &mut u16, text: &str, color: Option<Color>) -> crossterm::Result<()> {
        let color = color.unwrap_or(Color::White); // Default to white if no color is specified
        for line in text.split('\n') {
            let wrapped_lines = wrap(line, self.width as usize);
            for wrapped_line in wrapped_lines {
                execute!(stdout, SetForegroundColor(color))?;

                execute!(
                    stdout,
                    cursor::MoveTo(self.start_x, *start_y),
                    Clear(ClearType::CurrentLine),
                    Print(&wrapped_line)
                )?;

                execute!(stdout, ResetColor)?;
                
                *start_y += 1;
            }
        }
        Ok(())
    }

    
    pub fn show_page(&self, stdout: &mut io::Stdout, scroll_offset: u16) -> crossterm::Result<()> {
        match self.current_page {
            Page::Store => self.show_store(stdout),
            Page::About => self.show_about(stdout),
            Page::Landing => self.show_landing(stdout),
            Page::FAQ => self.show_faq(stdout, scroll_offset),
        }
    }
}
