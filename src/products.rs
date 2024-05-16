// src/products.rs
pub struct Product {
    pub name: &'static str,
    pub attributes: Vec<&'static str>,
    pub price: f64,
    pub description: &'static str,
    pub quantity: i16,
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
    ]
}
