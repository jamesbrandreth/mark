use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use tinytemplate::TinyTemplate;

#[derive(Debug, PartialEq, Eq)]
pub struct Element<'a> {
    pub template: String,
    pub content: HashMap<String, String>,
    pub parent: Option<&'a Element<'a>>,
    pub children: Vec<Element<'a>>,
}

impl<'a> Element<'a> {
    pub fn add_child(&'a  mut self, mut child: Element<'a>) {
        child.parent = Some(self);
        self.children.push(child);
    }
}

pub fn render_element(element: &Element, renderer: &TinyTemplate) -> String{
    let children = element.children.iter().map(|child: &Element| {
        render_element(child, renderer)
    }).collect();
    let mut context = element.content.clone();
    context.insert("children".to_string(), children);
    renderer.render(element.template.as_str(), &context).unwrap()
}
