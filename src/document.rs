use core::str::{SplitN};

pub trait HTML {
    fn to_html(&self) -> String;
}

pub trait MarkDown {
    fn from_markdown(s: &str) -> Self;
}

pub struct Document {
    pub body: Vec<Box<dyn HTML>>
}

impl MarkDown for Document {
    fn from_markdown(s: &str) -> Self {
        let mut body: Vec<Box<dyn HTML>> = vec![];
        for element in s.split("\n\n"){
            if element.chars().nth(0).unwrap() == '#' {
                body.push(Box::new(Heading::from_markdown(element)));
            } else {
                body.push(Box::new(Paragraph::from_markdown(element)));
            }
        }
        return Document{body: body};
    }
}

impl HTML for Document {
    fn to_html(&self) -> String {
        let mut s: String = String::from("");
        for el in &self.body {
            s = s + &el.to_html();
        }
        return format!("<!DOCTYPE html>
<html>
<head>
<title>This is a title</title>
</head>
<body>
{}
</body>
</html>", s);
    }
}

#[derive(Debug)]
pub struct Heading{
    pub level: u8,
    pub text: String
}

impl MarkDown for Heading {
    fn from_markdown(s: &str) -> Heading {
        let mut line: SplitN<&str> = s.splitn(2, " ");
        let hashes = line.next().unwrap();
        let text = line.next().unwrap();
        return Heading{level: hashes.chars().count() as u8, text:String::from(text)};
    }
}

impl HTML for Heading {
    fn to_html(&self) -> String {
        return format!("<h{}>{}</h{}>\n", self.level.to_string(), self.text, self.level.to_string());
    }
}

#[derive(Debug)]
pub struct Paragraph{
    pub text: String
}

impl MarkDown for Paragraph {
    fn from_markdown(s: &str) -> Paragraph {
        return Paragraph{text: String::from(s)}
    }
}

impl HTML for Paragraph {
    fn to_html(&self) -> String {
        return format!("<p>\n{}\n</p>\n", self.text);
    }
}
