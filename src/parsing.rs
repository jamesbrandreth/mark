use crate::document::Document;


use regex::Regex;


pub fn parse() {

    let re_quote = Regex::new(r"^{0,3}>").unwrap();
    let re_paragraph = Regex::new(r"\w+").unwrap();

    let s = String::from("
testing testing

> Testing
>
> 123
> > Another level
that carries on

test a new para
    ");
    let doc = Document{
        title: String::from("Title"),
        style: None,
        children: vec![],
    };
    let _current_node = &doc;
    for line in s.split("\n") {
//        println!("{}", line);
        if re_quote.is_match(&line) {
            println!("QUOTE: {line}");
        } else if re_paragraph.is_match(&line) {
            println!("PARAGRAPH: {line}");
        }
    }
}