use std::fs;

pub mod elements;
pub mod traits;
mod document;

use document::{Document};
use traits::{HTML, MarkDown};
use elements::heading::Heading;
use elements::paragraph::Paragraph;
use elements::thematic_break::ThematicBreak;
use elements::code::Code;

fn main() {
    let d = Document{children: vec![
        Box::new(Heading{level: 1, text: String::from("One")}),
        Box::new(Paragraph{text: String::from("One")}),
        Box::new(ThematicBreak{}),
        Box::new(Heading{level: 2, text: String::from("Two")}),
        Box::new(Paragraph{text: String::from("Two")}),
        Box::new(Code{text: String::from("Two three four")}),
        Box::new(Heading{level: 3, text: String::from("Three")}),
        Box::new(Paragraph{text: String::from("Three")}),
    ]};
    println!("{}", d.to_html());
}
