use std::fs;
use std::rc::Rc;
use std::cell::RefCell;
use tinytemplate::TinyTemplate;
use std::error::Error;
use std::collections::HashMap;
use serde::ser::{Serialize, Serializer, SerializeMap};

mod element;
use element::Element;


pub fn main() -> Result<(), Box<dyn Error>> {

    let templates: Vec<(String, String)> = fs::read_dir("./default").unwrap().map(|entry| {
        let path = entry.unwrap().path();
        let file_name = String::from(path.file_stem().unwrap().to_str().unwrap());
        let template = fs::read_to_string(&path).unwrap();
        (file_name, template)
    }).collect();

    let mut tt = TinyTemplate::new();
    for (name, template) in templates.iter() {
        tt.add_template(&name, &template);
    }

    let mut context: HashMap<String, String> = HashMap::new();
    context.insert(
            "body".to_string(),
            "Some test text element.".to_string(),
        );

    let mut root = Element{
        template: "paragraph".to_string(),
        content: HashMap::new(),
        parent: None,
        children: vec![],
    };

    root.children.push(Element{
        template: "text".to_string(),
        content: context,
        parent: None,
        children: vec![],
    });

    let rendered = element::render_element(&root, &tt);
    println!("{}", rendered);

    Ok(())
}
