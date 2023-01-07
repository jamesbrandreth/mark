use std::fs;

pub mod elements;
pub mod traits;
mod document;

use document::{Document};
use traits::{HTML, MarkDown};
use elements::heading::Heading;

fn main() {
    let d = Document{children: vec![
        Box::new(Heading{level: 1, text: String::from("One")}),
        Box::new(Heading{level: 2, text: String::from("Two")}),
        Box::new(Heading{level: 3, text: String::from("Three")}),
        Box::new(Heading{level: 4, text: String::from("Four")}),
        Box::new(Heading{level: 5, text: String::from("Five")}),
    ]};
    println!("{}", d.to_html());
}
