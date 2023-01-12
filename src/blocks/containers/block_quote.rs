
use std::rc::Rc;

use crate::traits::HTML;

pub struct BlockQuote {
    pub children: Vec<Box<dyn HTML>>,
    pub parent: Rc<Box<dyn HTML>>,
}

impl BlockQuote {
    pub const OPEN_RULE: &str = r"^\s[0,3]>";
    pub const CLOSE_RULE: &str = r"^\s[0,3]>";
}

impl HTML for BlockQuote {
    fn to_html(&self) -> String {
        let mut s: String = String::from("");
        for el in &self.children {
            s = s + &el.to_html();
        }
        return format!("<blockquote>{}</blockquote>", s);
    }
}
