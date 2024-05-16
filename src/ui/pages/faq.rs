use crossterm::{
    cursor,
    execute,
    style::{Color, Print, PrintStyledContent, ResetColor, SetForegroundColor, Stylize},
    terminal::{self, Clear, ClearType},
};
use textwrap::wrap;
use std::io::{self, Write};

use crate::ui::handler::UIState;

impl UIState {
    pub fn get_faqs(&self) -> Vec<(&str, &str)> {
        vec![
            (
                "How do I browse products in the terminal shop?",
                "You can browse products by using the 'list' command. This will display a list of available products with their descriptions and prices."
            ),
            (
                "How do I purchase a product?",
                "To purchase a product, use the 'buy' command followed by the product ID. You will be prompted to enter your payment details to complete the transaction using Stripe."
            ),
            (
                "Is my payment information secure?",
                "Yes, your payment information is secure. We use Stripe for payment processing, which ensures that your payment details are handled with top-notch security."
            ),
            (
                "Can I get a receipt for my purchase?",
                "Absolutely! After your purchase is complete, a receipt will be sent to your registered email address. You can also view your purchase history using the 'history' command."
            ),
            (
                "What if I encounter an issue during purchase?",
                "If you encounter any issues during the purchase process, you can contact our support team via the 'support' command. We are here to help you with any problems you might face."
            ),
        ]
    }

    pub fn show_faq(&self, stdout: &mut io::Stdout, scroll_offset: u16) -> crossterm::Result<()> {
        execute!(stdout, cursor::MoveTo(0, self.start_y + self.header_height), Clear(ClearType::FromCursorDown))?;

        let mut y = self.start_y + self.header_height;
        let available_height = terminal::size()?.1 - self.header_height - self.footer_height;

        let faqs = self.get_faqs();

        // Flatten the FAQ content into lines, considering wrapped lines
        let mut lines: Vec<(String, Color)> = Vec::new();
        for (question, answer) in faqs {
            for line in wrap(question, self.width as usize) {
                lines.push((line.to_string(), Color::White));
            }
            lines.push(("".to_string(), Color::White)); // Add a blank line between Q&A pairs
            for line in wrap(answer, self.width as usize) {
                lines.push((line.to_string(), Color::DarkGrey));
            }
            lines.push(("".to_string(), Color::White)); // Add a blank line between Q&A pairs
            lines.push(("".to_string(), Color::White)); // Add a blank line between Q&A pairs
        }

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
        self.render_scrollbar(stdout, scroll_offset, lines.len() as u16, available_height)?;


        Ok(())
    }

    fn render_scrollbar(&self, stdout: &mut io::Stdout, scroll_offset: u16, content_height: u16, available_height: u16) -> crossterm::Result<()> {
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
