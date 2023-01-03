mod document;
use document::{HTML, Document, Heading, Paragraph};

fn main() {
    let h:Heading = Heading{level: 1, text: String::from("TEST")};
    let p:Paragraph = Paragraph{text: String::from("testing.")};
    let d:Document = Document{body: vec![Box::new(h),Box::new(p)]};
    println!("{}", d.to_html());
}
