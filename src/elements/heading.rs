use core::str::{SplitN};


use crate::traits::{HTML, MarkDown};

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
        return Heading{
            level: hashes.chars().count() as u8,
            text:String::from(text)
        };
    }
}

impl HTML for Heading {
    fn to_html(&self) -> String {
        return format!(
                "<h{}>{}</h{}>",
            self.level.to_string(),
            self.text,
            self.level.to_string()
        );
    }
}