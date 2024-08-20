mod products;
mod ui;

use crossterm::event::Event;
use products::get_products;
use textwrap::wrap;
use ui::header::draw_header;
use ui::footer::draw_footer;
use ui::handler::{UIState, Page};
use ui::loader::show_loading_screen;

use log::Level;
use log::info;

use crossterm::{
    event::{self, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{self, ClearType},
};
use std::io::{self, Write};

fn main() -> crossterm::Result<()> {

    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    let (mut cols, mut rows) = terminal::size()?;
    let mut scroll_offset: u16 = 0;

    // Show the loading screen
    show_loading_screen(&mut stdout)?;

    // Clear the screen after the loading screen
    execute!(stdout, terminal::Clear(ClearType::All), crossterm::cursor::Hide)?;

    let products: Vec<products::Product> = get_products();
    let mut ui_state: UIState = UIState::new(products, cols, rows);

    ui_state.header_height = draw_header(&mut stdout, ui_state.start_x, ui_state.start_y, &ui_state.current_page)?;
    draw_footer(&mut stdout, ui_state.start_x, rows, ui_state.width)?;
    ui_state.show_page(&mut stdout,scroll_offset)?;
    
    
    let pages = vec![Page::Store, Page::About, Page::FAQ];
    let mut current_page_index = pages.len();
    
    loop {
        if event::poll(std::time::Duration::from_millis(500))? {
            match event::read()? {
                Event::Key(KeyEvent { code, modifiers }) => {
                    match (code, modifiers) {
                        (KeyCode::Left, KeyModifiers::NONE) => {
                            if current_page_index > 0 {
                                current_page_index -= 1;
                            } else {
                                current_page_index = pages.len() - 1; // to the last page
                            }
                            scroll_offset = 0;
                            ui_state.current_page = pages[current_page_index].clone();
                            ui_state.header_height = draw_header(&mut stdout, ui_state.start_x, ui_state.start_y, &ui_state.current_page)?;
                            ui_state.show_page(&mut stdout, scroll_offset)?;
                        }
                        (KeyCode::Right, KeyModifiers::NONE) => {
                            if current_page_index < pages.len() - 1 {
                                current_page_index += 1;
                            } else {
                                current_page_index = 0; // to the first page
                            }
                            scroll_offset = 0;
                            ui_state.current_page = pages[current_page_index].clone();
                            ui_state.header_height = draw_header(&mut stdout, ui_state.start_x, ui_state.start_y, &ui_state.current_page)?;
                            ui_state.show_page(&mut stdout, scroll_offset)?;
                        }
                        (KeyCode::Up, KeyModifiers::NONE) => {
                            if scroll_offset > 0 {
                                scroll_offset -= 1;
                                ui_state.header_height = draw_header(&mut stdout, ui_state.start_x, ui_state.start_y, &ui_state.current_page)?;
                                ui_state.show_page(&mut stdout, scroll_offset)?;
                            }
                        }
                        (KeyCode::Down, KeyModifiers::NONE) => {
                            let max_scroll = calculate_max_scroll(&ui_state, rows);
                            if scroll_offset < max_scroll {
                                scroll_offset += 1;
                                ui_state.header_height = draw_header(&mut stdout, ui_state.start_x, ui_state.start_y, &ui_state.current_page)?;
                                ui_state.show_page(&mut stdout, scroll_offset)?;
                            }
                        }
                        (KeyCode::Char('c'), KeyModifiers::CONTROL) | (KeyCode::Char('q'), KeyModifiers::NONE) => break,
                        _ => {}
                    }
                    draw_footer(&mut stdout, ui_state.start_x, rows, ui_state.width)?;
                }
                Event::Resize(new_cols, new_rows) => {
                    cols = new_cols;
                    rows = new_rows;
                    ui_state.update_dimensions(cols, rows);
                    execute!(stdout, terminal::Clear(ClearType::All))?;
                    ui_state.header_height = draw_header(&mut stdout, ui_state.start_x, ui_state.start_y, &ui_state.current_page)?;
                    draw_footer(&mut stdout, ui_state.start_x, rows, ui_state.width)?;
                    ui_state.show_page(&mut stdout, scroll_offset)?;
                }
                _ => {}
            }
        }
    }
    execute!(stdout, crossterm::cursor::Show)?;
    terminal::disable_raw_mode()?;
    Ok(())
}

fn calculate_max_scroll(ui_state: &UIState, terminal_height: u16) -> u16 {
    let faqs = ui_state.get_faqs();
    let mut total_lines = 0;

    for (question, answer) in faqs {
        total_lines += wrap(question, ui_state.width as usize).len() as u16;
        total_lines += 1; // Space between lines
        total_lines += wrap(answer, ui_state.width as usize).len() as u16;
        total_lines += 2; // Space between items
    }

    let available_height = terminal_height - ui_state.header_height - ui_state.footer_height;
    if total_lines > available_height {
        total_lines - available_height
    } else {
        0
    }
}