
pub trait HTML {
    fn to_html(&self) -> String;
}

pub struct Document {
    pub body: Vec<Box<dyn HTML>>
}

impl HTML for Document {
    fn to_html(&self) -> String {
        let mut s: String = String::from("");
        for el in &self.body {
            s = s + &el.to_html();
        }
        return s;
    }
}

#[derive(Debug)]
pub struct Heading{
    pub level: u8,
    pub text: String
}

impl HTML for Heading {
    fn to_html(&self) -> String {
        return format!("<h{}>{}</h>\n", self.level.to_string(), self.text);
    }
}

#[derive(Debug)]
pub struct Paragraph{
    pub text: String
}

impl HTML for Paragraph {
    fn to_html(&self) -> String {
        return format!("<p>\n{}\n</p>", self.text);
    }
}
