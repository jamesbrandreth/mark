use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use regex::Regex;

use crate::element::Element;

fn new_document(title: &str) -> Element {
    Element::new(
            "document".to_string(),
            HashMap::from([
                ("title".to_string(), title.to_string()),
            ]),
    )
}

fn new_text(s: &str) -> Element {
    Element::new(
            "text".to_string(),
            HashMap::from([
                ("body".to_string(), s.to_string()),
            ]),
    )
}

fn new_paragraph(s: &str) -> Element {
    Element::new(
            "paragraph".to_string(),
            HashMap::from([]),
    )
}

pub fn parse(s: &str) -> Element {

    let paragraph_rule = Regex::new(r"^\w").unwrap();

    let mut root = new_document("test title");
    let mut node: Element = root.clone();

    for line in s.lines() {
        println!("{}", line);
        if paragraph_rule.is_match(line) {
            node.add_child(new_paragraph(line));
        }
    }

   return root;
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::element::Element;
    use super::parse;
    use std::rc::Rc;
    use std::cell::RefCell;

    #[test]
    fn test_paragraph() {

        let mut doc = Element::new(
                "paragraph".to_string(),
                HashMap::from([
                    ("title".to_string(), "test".to_string()),
                    ("syle".to_string(), "default".to_string()),
                ]),
        );
        let paragraph = Element::new(
                "paragraph".to_string(),
                HashMap::from([]),
        );
        doc.add_child(paragraph.clone());
//        assert_eq!(parse("tests"), doc);
    }
}