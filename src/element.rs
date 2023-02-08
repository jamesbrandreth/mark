use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use tinytemplate::TinyTemplate;

pub struct Element {
    pub template: String,
    pub content: HashMap<String, String>,
    pub parent: Option<Rc<RefCell<Box<Element>>>>,
    pub children: Vec<Element>,
}

pub fn render_element(element: &Element, renderer: &TinyTemplate) -> String{
    let children = element.children.iter().map(|child: &Element| {
        render_element(child, renderer)
    }).collect();
    let mut context = element.content.clone();
    context.insert("children".to_string(), children);
    renderer.render(element.template.as_str(), &context).unwrap()
}
