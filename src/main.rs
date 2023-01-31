use std::fs;
use tinytemplate::TinyTemplate;
use std::error::Error;
use std::collections::HashMap;


pub fn main() -> Result<(), Box<dyn Error>> {

    let mut templates: Vec<(String, String)> = vec![];
    let mut tt = TinyTemplate::new();

    for entry in fs::read_dir("./default").unwrap() {
        let path = entry.unwrap().path();
        let file_name = String::from(path.file_stem().unwrap().to_str().unwrap());
        let template = fs::read_to_string(&path).unwrap();
        templates.push((file_name, template));
    }

    for (name, template) in templates.iter() {
        tt.add_template(&name, &template);
    }
    let mut context: HashMap<String, String> = HashMap::new();
    context.insert(
            "name".to_string(),
            "bob".to_string(),
        );
    let rendered = tt.render("paragraph", &context)?;
    println!("{}", rendered);

    Ok(())
}