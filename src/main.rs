use std::fs;

pub mod blocks;
pub mod traits;
mod document;

use document::{Document};
use traits::{HTML, MarkDown};
use blocks::leaves::heading::Heading;
use blocks::leaves::paragraph::Paragraph;
use blocks::leaves::thematic_break::ThematicBreak;
use blocks::leaves::code::Code;
use blocks::leaves::html::RawHTML;
use blocks::containers::block_quote::BlockQuote;


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
        Box::new(RawHTML{text: String::from("<div>Three</div>")}),
        Box::new(Heading{level: 4, text: String::from("Four")}),
        Box::new(BlockQuote{children: vec![Box::new(Paragraph{text: String::from("<div>Three</div>")})]}),
    ]};
    println!("{}", d.to_html());
}
