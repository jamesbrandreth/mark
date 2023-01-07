use regex::Regex;

use crate::traits::{HTML, MarkDown};
use crate::blocks::leaves::heading::Heading;
use crate::blocks::leaves::paragraph::Paragraph;

pub struct Document {
    pub children: Vec<Box<dyn HTML>>,
}

impl MarkDown for Document {
    fn from_markdown(s: &str) -> Document {

        let re_blankline = Regex::new(r"^\s*$").unwrap();
        let re_heading_line = Regex::new(r"^(#)+\s*(\w)\s*#*$").unwrap();

        let mut root = Document{children: vec![]};
        let mut node = &mut root;
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
        return format!("<!DOCTYPE html><html><head><title>This is a title</title></head><body>{}</body></html>", s);
    }
}

