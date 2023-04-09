use std::fs;
use std::error::Error;
use std::collections::HashMap;
use tinytemplate::TinyTemplate;

mod element;
use element::Element;

mod parser;


pub fn main() -> Result<(), Box<dyn Error>> {

    let templates: Vec<(String, String)> = glob::glob("./templates/default/*.html").unwrap().map(|entry| {
        let path = entry.unwrap();
        let file_name = String::from(path.file_stem().unwrap().to_str().unwrap());
        let template = fs::read_to_string(&path).unwrap();
        (file_name, template)
    }).collect();

    let styles: String = glob::glob("./templates/default/*.css").unwrap().map(|entry| {
        fs::read_to_string(entry.unwrap().as_path()).unwrap()
    }).collect::<String>();

    let mut tt = TinyTemplate::new();
    tt.set_default_formatter(&tinytemplate::format_unescaped);
    for (name, template) in templates.iter() {
        match tt.add_template(&name, &template) {
            Ok(_) => {},
            Err(_) => {panic!("Error loading template: {}", template)},
        };
    }

    let root = Element::new("document".to_string(),
            HashMap::from([
                ("title".to_string(),"test".to_string()),
                ("style".to_string(),"default".to_string()),
            ]));
    let mut current_node: Element = root.clone();

    let paragraph = Element::new(
            "paragraph".to_string(),
            HashMap::new(),
    );
    current_node.add_child(paragraph.clone());
    current_node = paragraph;

    println!("{}", root);
    println!("{}", root.render(&tt));

    Ok(())
}
