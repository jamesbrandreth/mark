use std::fs;
use std::error::Error;
use std::collections::HashMap;
use tinytemplate::TinyTemplate;

mod element;
use element::Element;


pub fn main() -> Result<(), Box<dyn Error>> {

    let templates: Vec<(String, String)> = fs::read_dir("./templates/default").unwrap().map(|entry| {
        let path = entry.unwrap().path();
        let file_name = String::from(path.file_stem().unwrap().to_str().unwrap());
        let template = fs::read_to_string(&path).unwrap();
        (file_name, template)
    }).collect();

    let mut tt = TinyTemplate::new();
    tt.set_default_formatter(&tinytemplate::format_unescaped);
    for (name, template) in templates.iter() {
        match tt.add_template(&name, &template) {
            Ok(_) => {},
            Err(_) => {panic!("Error loading template: {}", template)},
        };
    }

    let mut context: HashMap<String, String> = HashMap::new();
    context.insert(
            "body".to_string(),
            "Some test text element.".to_string(),
        );

    let root = Element{
        template: "document".to_string(),
        content: HashMap::from([
            ("title".to_string(), "A Test Document".to_string()),
        ]),
        parent: None,
        children: vec![Element{
            template: "paragraph".to_string(),
            content: HashMap::new(),
            parent: None,
            children: vec![
                Element{
                    template: "text".to_string(),
                    content: context,
                    parent: None,
                    children: vec![],
                }
            ],
        }],
    };


    let rendered = element::render_element(&root, &tt);
    println!("{}", rendered);

    Ok(())
}
