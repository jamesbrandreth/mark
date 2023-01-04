use std::fs;

mod document;
use document::{HTML, MarkDown, Document};

fn main() {
    let file_contents = fs::read_to_string("./test/test.md").expect("Help!");
    let d:Document = Document::from_markdown(file_contents.as_str());
    println!("{}", d.to_html());
}
