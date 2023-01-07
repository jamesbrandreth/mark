use crate::traits::HTML;

pub struct BlockQuote{
    pub children: Vec<Box<dyn HTML>>,
}

impl HTML for BlockQuote {
    fn to_html(&self) -> String {
        let mut s: String = String::from("");
        for el in &self.children {
            s = s + &el.to_html();
        }
        return format!("<blockquote>{}<blockquote>", s);
    }
}