mod document;
use document::{HTML, MarkDown, Document, Heading, Paragraph};

fn main() {
    let h:Heading = Heading::from_markdown("## Test Heading");
    let p:Paragraph = Paragraph::from_markdown("test.");
    let d:Document = Document{body: vec![Box::new(h), Box::new(p)]};
    println!("{}", d.to_html());
}
