use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use crate::element::Element;
use regex::Regex;

fn new_document(title: &str) -> Element {
    Element {
        template: "document".to_string(),
        content: HashMap::from([
            ("title".to_string(), title.to_string()),
            ]),
        parent: None,
        children: vec![],
    }
}

fn new_text<'a>(s: &str, parent: Option<&'a Element<'a>>) -> Element<'a> {
    Element {
        template: "text".to_string(),
        content: HashMap::from([
            ("body".to_string(), s.to_string()),
            ]),
        parent: parent,
        children: vec![],
    }
}

fn new_paragraph<'a>(s: &str) -> Element<'a> {
    Element {
        template: "paragraph".to_string(),
        content: HashMap::from([]),
        parent: None,
        children: vec![],
    }
}

pub fn parse(s: &str) -> Element {

    let paragraph_rule = Regex::new(r"^\w").unwrap();

    let mut stack: Vec<&mut Element> = vec![];
    let mut root: Element = new_document("test title");
    stack.push(&mut root);

    for line in s.lines() {
        println!("{}", line);
        println!("{}", paragraph_rule.is_match(line)) ;

        if paragraph_rule.is_match(line) {
            stack.last_mut().unwrap().add_child(
                    new_paragraph(line)
            );
        }
    }

    return root;
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::element::Element;
    use super::parse;

    #[test]
    fn test_paragraph() {
        assert_eq!(
            parse("tests\n\ntesting\n"),
            Element{
                template: "paragraph".to_string(),
                content: HashMap::from([]),
                parent: None,
                children: vec![
                    Element{
                        template: "text".to_string(),
                        content: HashMap::from([
                            ("text".to_string(), "tests".to_string()),
                            ]),
                        parent: None,
                        children: vec![],
                    },
                ],
            }
        );
    }
}