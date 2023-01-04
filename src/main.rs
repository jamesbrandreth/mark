mod document;
use document::{HTML, MarkDown, Document, Heading, Paragraph};

fn main() {
    let d:Document = Document::from_markdown("# Heading\n\nText.");
    println!("{}", d.to_html());
}
