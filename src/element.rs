use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use tinytemplate::TinyTemplate;

#[derive(Debug, PartialEq, Eq)]

struct ElementData {
    template: String,
    content: HashMap<String, String>,
    parent: Option<Element>,
    children: Vec<Element>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Element {
    data: Rc<RefCell<ElementData>>,
}

impl Element {
    pub fn new(template: String, content: HashMap<String, String>) -> Self {
        let node_data = Rc::new(RefCell::new(ElementData {
            template: template,
            content: content,
            parent: None,
            children: vec![],
        }));

        Self { data: node_data }
    }

    pub fn parent(&self) -> Option<Element> {
        self.data.borrow().parent.clone()
    }

    fn set_parent(&mut self, parent: Element) {
        self.data.borrow_mut().parent = Some(parent);
    }

    pub fn add_child(&mut self, mut child: Element) {
        child.set_parent(self.clone());
        self.data.borrow_mut().children.push(child);
    }

    pub fn render(&self, renderer: &TinyTemplate) -> String{
        let children = self
            .data
            .borrow()
            .children
            .iter()
            .map(|child: &Element| {
                child.render(renderer)
            })
            .collect();
        let mut context = self.data.borrow().content.clone();
        context.insert("children".to_string(), children);
        renderer.render(self.data.borrow().template.as_str(), &context).unwrap()
    }
}

impl std::fmt::Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let element_data = self.data.borrow();
        let mut children: String = element_data
            .children
            .iter()
            .map(|child| format!("{}\n", child))
            .collect();
        children = children
            .lines()
            .map(|line| format!("\n  {}", line))
            .collect();
        write!(f, "{}{}", element_data.template, format!("{}", children))
    }
}


