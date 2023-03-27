use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use crate::element::Element;

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

fn new_text(s: &str, parent: Option<Rc<RefCell<Box<Element>>>>) -> Element {
    Element {
        template: "text".to_string(),
        content: HashMap::from([
            ("body".to_string(), s.to_string()),
            ]),
        parent: parent,
        children: vec![],
    }
}

fn new_paragraph(s: &str, parent: Option<Rc<RefCell<Box<Element>>>>) -> Element {
    Element {
        template: "paragraph".to_string(),
        content: HashMap::from([]),
        parent: parent,
        children: vec![],
    }
}

pub fn parse(s: &str) -> Element {

    let mut stack: Vec<Rc<RefCell<Box<Element>>>> = vec![];
    let mut root: Element = new_document("test title");
    stack.push(Rc::new(RefCell::new(Box::new(root))));

    for line in String::from(s).lines() {
        println!("{}", line);
        for element in stack {
            
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
            parse("tests\n"),
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