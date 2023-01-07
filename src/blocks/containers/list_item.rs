use crate::traits::HTML;

pub struct ListItem{
    pub children: Vec<Box<dyn HTML>>,
}

impl HTML for ListItem {
    fn to_html(&self) -> String {
        let mut s: String = String::from("");
        for el in &self.children {
            s = s + &el.to_html();
        }
        return format!("<li>{}</li>", s);
    }
}