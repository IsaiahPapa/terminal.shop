use crossterm::{
    cursor,
    execute,
    style::{SetForegroundColor, Color, Print, ResetColor, SetAttribute, Attribute},
    ExecutableCommand,
};
use std::io::{self, Write};

use crate::ui::handler::Page;

const NAV_ITEM_WIDTH: usize = 10;
const CART_WIDTH: usize = 20;

pub fn center_text(text: &str, width: usize) -> String {
    let padding = (width.saturating_sub(text.len())) / 2;
    let padded_text = format!("{:padding$}{}{:padding$}", "", text, "", padding = padding);
    padded_text
}


pub fn nav_item(label: &str, highlighted: bool) -> String {
    if highlighted {
        center_text(&format!("{}", label), NAV_ITEM_WIDTH)
    } else {
        center_text(label, NAV_ITEM_WIDTH)
    }
}

pub fn cart(total: f64, item_count: usize) -> String {
    center_text(&format!("cart ${:.2} [{}]", total, item_count), CART_WIDTH)
}

pub fn draw_header(stdout: &mut io::Stdout, start_x: u16, start_y: u16, current_page: &Page) -> crossterm::Result<u16> {
    execute!(stdout, cursor::MoveTo(start_x, start_y), SetForegroundColor(Color::DarkGrey))?;

    let logo = nav_item("terminal", true);
    let shop = nav_item("shop", matches!(current_page, Page::Store));
    let about = nav_item("about", matches!(current_page, Page::About));
    let faq = nav_item("faq", matches!(current_page, Page::FAQ)); // Assuming there's a FAQ page
    let cart = cart(0.0, 0); // Update with actual cart data if available

    let components = vec![logo, shop, about, faq, cart];
    let header_height = 4; // Box with 3 lines: top border, content, bottom border
    let mut x = start_x + 1;

    let mut top_border = String::from("┌");
    let mut bottom_border = String::from("└");

    for (i, component) in components.iter().enumerate() {
        top_border.push_str(&"─".repeat(component.len()));
        bottom_border.push_str(&"─".repeat(component.len()));
        if i < components.len() - 1 {
            top_border.push('┬');
            bottom_border.push('┴');
        }
    }

    top_border.push('┐');
    bottom_border.push('┘');

    execute!(stdout, cursor::MoveTo(start_x, start_y), Print(&top_border))?;


    // Move to the content line
    execute!(stdout, cursor::MoveTo(start_x, start_y + 1))?;
    execute!(stdout, Print("│"))?;

    // Print each part with appropriate formatting
    for (i, component) in components.iter().enumerate() {
        execute!(stdout, cursor::MoveTo(x, start_y + 1))?;
        if (i==0 || i == 1 && matches!(current_page, Page::Store)) ||
           (i == 2 && matches!(current_page, Page::About)) ||
           (i == 3 && matches!(current_page, Page::FAQ)) { // Check if the component is highlighted
            execute!(stdout, SetAttribute(Attribute::Bold), SetForegroundColor(Color::White), Print(component), SetAttribute(Attribute::Reset), ResetColor, SetForegroundColor(Color::DarkGrey))?;
        } else {
            execute!(stdout, Print(component))?;
        }
        x += component.len() as u16; // Move cursor to the next component

        // Print separators between components
        if i < components.len() - 1 {
            execute!(stdout, cursor::MoveTo(x, start_y + 1), Print("│"))?;
            x += 1;
        }
    }

    // Move to the end of the content line and print the closing box character
    execute!(stdout, cursor::MoveTo(x - 1, start_y + 1), Print(" │"))?;

    // Draw the bottom border of the box
    execute!(stdout, cursor::MoveTo(start_x, start_y + 2), Print(&bottom_border))?;

    execute!(stdout, ResetColor)?;

    Ok(header_height)
}
