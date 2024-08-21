use crossterm::style::Color;
use textwrap::wrap;

use crate::ui::handler::UIState;

// src/products.rs
pub struct Product {
    pub name: &'static str,
    pub attributes: Vec<&'static str>,
    pub price: f64,
    pub description: &'static str,
    pub quantity: i16,
}

pub enum ColorScheme {
    TitleColor,
    NameColor,
    AttributesColor,
    PriceColor,
    DescriptionColor,
    QuantityColor
}

impl ColorScheme {
    pub fn to_color(&self) -> Color {
        match self {
            ColorScheme::TitleColor => Color::White,
            ColorScheme::NameColor => Color::White,
            ColorScheme::AttributesColor => Color::DarkGrey,
            ColorScheme::PriceColor => Color::Rgb { r: 255, g: 90, b: 0 },
            ColorScheme::DescriptionColor => Color::DarkGrey,
            ColorScheme::QuantityColor => Color::White,
        }
    }
}

pub fn get_products() -> Vec<Product> {
    vec![
        Product {
            name: "nil blend coffee",
            attributes: vec!["whole bean", "medium roast"],
            price: 25.0,
            description: "Dive into the rich taste of Nil, our delicious semi-sweet coffee with notes of chocolate, peanut butter, and a hint of fig.",
            quantity: 362,
        },
        Product {
            name: "Eclipse Dark Roast",
            attributes: vec!["ground", "dark roast"],
            price: 28.0,
            description: "Experience the deep, bold flavors of Eclipse, a dark roast coffee with rich notes of dark chocolate, molasses, and a hint of smoke.",
            quantity: 415,
        },
        Product {
            name: "Sunrise Light Roast",
            attributes: vec!["whole bean", "light roast"],
            price: 23.0,
            description: "Wake up to the bright and lively flavors of Sunrise, a light roast coffee with crisp citrus, floral notes, and a smooth finish.",
            quantity: 300,
        },
        Product {
            name: "Harvest Pumpkin Spice",
            attributes: vec!["ground", "medium roast", "flavored"],
            price: 27.5,
            description: "Embrace the season with Harvest, a pumpkin spice flavored coffee with warm cinnamon, nutmeg, and clove notes, perfectly balanced with a smooth medium roast.",
            quantity: 150,
        },
        Product {
            name: "Mocha Hazelnut Delight",
            attributes: vec!["whole bean", "medium roast", "flavored"],
            price: 30.0,
            description: "Indulge in the sweet and nutty Mocha Hazelnut Delight, a medium roast coffee with luscious chocolate and hazelnut flavors, complemented by a creamy finish.",
            quantity: 275,
        }
        
    ]
}

pub fn get_products_lines(ui_state: &UIState) -> Vec<(String, Color)> {
    let mut lines: Vec<(String, Color)> = Vec::new();
    let products = get_products();

    for product in products.iter() {
        lines.push(("─".repeat(ui_state.width as usize), ColorScheme::TitleColor.to_color()));

        for line in wrap(&product.name, ui_state.width as usize) {
            lines.push((line.to_string(), ColorScheme::NameColor.to_color()));
        }
        lines.push(("".to_string(), Color::White)); // Add a blank line between Q&A pairs

        let attributes = product.attributes.join(" | ");
        for line in wrap(&format!("{}", attributes), ui_state.width as usize) {
            lines.push((line.to_string(), ColorScheme::AttributesColor.to_color()));
        }
        lines.push(("".to_string(), Color::White)); // Add a blank line between Q&A pairs

        for line in wrap(&format!("${:.2}", product.price), ui_state.width as usize) {
            lines.push((line.to_string(), ColorScheme::PriceColor.to_color()));
        }
        lines.push(("".to_string(), Color::White)); // Add a blank line between Q&A pairs

        for line in wrap(&format!("{}", product.description), ui_state.width as usize) {
            lines.push((line.to_string(), ColorScheme::DescriptionColor.to_color()));
        }
        lines.push(("".to_string(), Color::White)); // Add a blank line between Q&A pairs

        for line in wrap(&format!("{} Bags left", product.quantity), ui_state.width as usize) {
            lines.push((line.to_string(), ColorScheme::QuantityColor.to_color()));
        }
        lines.push(("".to_string(), Color::White)); // Add a blank line between Q&A pairs
        lines.push(("".to_string(), Color::White)); // Add a blank line between Q&A pairs
    }
    return lines;
}