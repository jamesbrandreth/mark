use regex::Regex;

use crate::traits::{HTML, MarkDown};
use crate::blocks::leaves::heading::Heading;


pub struct Document {
    pub title: String,
    pub style: Option<String>,
    pub children: Vec<Box<dyn HTML>>,
}

impl MarkDown for Document {
    fn from_markdown(s: &str) -> Document {

        let re_blankline = Regex::new(r"^\s*$").unwrap();
        let re_heading_line = Regex::new(r"^(#)+\s*(\w)\s*#*$").unwrap();

        let mut root = Document{title: String::from(""), style: None, children: vec![]};
        let node = &mut root;
        for line in s.split("\n") {

            if re_blankline.is_match(line) {
                break;
            } else if re_heading_line.is_match(line) {
                let caps = re_heading_line.captures(line).unwrap();
                let level = caps.get(1).unwrap().as_str().chars().count() as u8;
                let text = String::from(caps.get(2).unwrap().as_str());
                node.children.push(Box::new(Heading{level: level, text: text}));
            } else {

            }
        }
        return root;
    }
}

impl HTML for Document {
    fn to_html(&self) -> String {
        let mut s: String = String::from("");
        for el in &self.children {
            s = s + &el.to_html();
        }
        let style: String = match &self.style {
            None => String::from(""),
            Some(s) => format!("<link rel=\"stylesheet\" href=\"{}\">", s),
        };
        return format!("<!DOCTYPE html><html><head>{}<title>{}</title></head><body>{}</body></html>", style,  self.title, s);
    }
}

