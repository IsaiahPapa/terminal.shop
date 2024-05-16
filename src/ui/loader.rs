use crossterm::{
    cursor,
    execute,
    style::{Color, Print, ResetColor, SetAttribute, SetForegroundColor, Attribute},
    terminal::{Clear, ClearType},
};
use rand::Rng;
use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

const BOX_WIDTH: usize = 40;
const BOX_HEIGHT: usize = 20;
const INNER_BOX_WIDTH: usize = 20;
const INNER_BOX_HEIGHT: usize = 3;
const CHARACTERS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
const LOADING_TEXT: &str = "LOADING STORE";

fn generate_random_string(length: usize) -> String {
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARACTERS.len());
            CHARACTERS.chars().nth(idx).unwrap()
        })
        .collect()
}

pub fn show_loading_screen(stdout: &mut io::Stdout) -> crossterm::Result<()> {
    execute!(stdout, Clear(ClearType::All), cursor::Hide)?;

    let (term_width, term_height) = crossterm::terminal::size()?;
    let start_x = (term_width.saturating_sub(BOX_WIDTH as u16)) / 2;
    let start_y = (term_height.saturating_sub(BOX_HEIGHT as u16)) / 2;

    let inner_start_x = (BOX_WIDTH.saturating_sub(INNER_BOX_WIDTH)) / 2;
    let inner_start_y = (BOX_HEIGHT.saturating_sub(INNER_BOX_HEIGHT)) / 2;

    for _ in 0..8 { // Show the box for 2 seconds (8 * 250ms)
        let random_text = generate_random_string(BOX_WIDTH * BOX_HEIGHT);
        execute!(stdout, cursor::MoveTo(start_x, start_y))?;

        for (i, line) in random_text.chars().collect::<Vec<char>>().chunks(BOX_WIDTH).enumerate() {
            let line_str = String::from_iter(line);
            if i >= inner_start_y && i < inner_start_y + INNER_BOX_HEIGHT {
                let prefix = &line_str[..inner_start_x.min(line_str.len())];
                let suffix = &line_str[(inner_start_x + INNER_BOX_WIDTH).min(line_str.len())..];
                if i == inner_start_y + 1 {
                    let text_start_x = inner_start_x + (INNER_BOX_WIDTH - LOADING_TEXT.len()) / 2;
                    let mut middle_line = String::new();
                    // middle_line.push_str(prefix);
                    middle_line.push_str(&" ".repeat(text_start_x.saturating_sub(inner_start_x)));
                    middle_line.push_str(LOADING_TEXT);
                    middle_line.push_str(&" ".repeat(INNER_BOX_WIDTH.saturating_sub(text_start_x + LOADING_TEXT.len() - inner_start_x)));
                    // middle_line.push_str(suffix);
                    execute!(
                        stdout,
                        cursor::MoveTo(start_x, start_y + i as u16),
                        SetForegroundColor(Color::DarkGrey),
                        Print(&prefix),
                        SetForegroundColor(Color::White),
                        SetAttribute(Attribute::Bold),
                        Print(middle_line),
                        ResetColor,
                        SetAttribute(Attribute::Reset),
                        SetForegroundColor(Color::DarkGrey),
                        Print(&suffix),
                    )?;
                } else {
                    let inner_box_line = format!(
                        "{}{}{}",
                        prefix,
                        " ".repeat(INNER_BOX_WIDTH),
                        suffix
                    );
                    execute!(
                        stdout,
                        cursor::MoveTo(start_x, start_y + i as u16),
                        SetForegroundColor(Color::DarkGrey),
                        Print(inner_box_line),
                        ResetColor
                    )?;

                }
            } else {
                execute!(
                    stdout,
                    cursor::MoveTo(start_x, start_y + i as u16),
                    SetForegroundColor(Color::DarkGrey),
                    Print(line_str),
                    ResetColor
                )?;
            }
        }

        io::stdout().flush().unwrap();
        sleep(Duration::from_millis(250));
    }

    execute!(stdout, Clear(ClearType::All), cursor::Show)?;

    Ok(())
}

