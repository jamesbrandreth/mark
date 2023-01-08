pub mod blocks;
pub mod traits;
mod document;

use document::Document;
use traits::HTML;
use blocks::{
    leaves::{
        heading::Heading,
        paragraph::Paragraph,
        thematic_break::ThematicBreak,
        code::Code,
        html::RawHTML,
        link::Link,
},
    containers::{
        block_quote::BlockQuote,
        list_item::ListItem,
        unordered_list::UnorderedList,
        ordered_list::OrderedList
    }
};



fn main() {
    let d = Document{
        title: String::from("Title"),
        children: vec![

        Box::new(Heading{level: 1, text: String::from("Thematic Break")}),
        Box::new(ThematicBreak{}),

        Box::new(Heading{level: 1, text: String::from("Heading")}),
        Box::new(Heading{level: 2, text: String::from("Heading")}),
        Box::new(Heading{level: 3, text: String::from("Heading")}),
        Box::new(Heading{level: 4, text: String::from("Heading")}),
        Box::new(Heading{level: 5, text: String::from("Heading")}),

        Box::new(Heading{level: 1, text: String::from("Code")}),
        Box::new(Code{text: String::from("print(\"hello world\")")}),

        Box::new(Heading{level: 1, text: String::from("HTML")}),
        Box::new(RawHTML{text: String::from("<div>Three</div>")}),

        Box::new(Heading{level: 1, text: String::from("Link")}),
        Box::new(Link{label: String::from("README.md"), destination: String::from("README.md")}),

        Box::new(Heading{level: 1, text: String::from("Paragraph")}),
        Box::new(Paragraph{text: String::from("Lorem Ipsem")}),

        Box::new(Heading{level: 1, text: String::from("Block Quote")}),
        Box::new(BlockQuote{children: vec![Box::new(Paragraph{text: String::from("Lorem Ipsem.")})]}),

        Box::new(Heading{level: 1, text: String::from("Unordered List")}),
        Box::new(UnorderedList{children: vec![ListItem{children: vec![Box::new(Paragraph{text: String::from("Test list")})]}]}),

        Box::new(Heading{level: 1, text: String::from("Ordered List")}),
        Box::new(OrderedList{
            start: 1,
            children: vec![
                ListItem{children: vec![
                    Box::new(Paragraph{text: String::from("Test list")})
                ]},
                ListItem{children: vec![
                    Box::new(OrderedList{
                        start: 1,
                        children: vec![
                            ListItem{children: vec![
                                Box::new(Paragraph{text: String::from("Sub list")})
                            ]},
                            ListItem{children: vec![
                                Box::new(Paragraph{text: String::from("Sub list")})
                            ]},
                        ]
                    })
                ]},
            ]
        }),
    ]};
    println!("{}", d.to_html());
}
