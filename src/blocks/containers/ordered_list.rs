use crate::traits::HTML;
use crate::blocks::containers::list_item::ListItem;


pub struct OrderedList{
    pub start: u8,
    pub children: Vec<ListItem>,
}

impl HTML for OrderedList {
    fn to_html(&self) -> String {
        let mut s: String = String::from("");
        for el in &self.children {
            s = s + &el.to_html();
        }
        return format!("<ol start={}>{}</ol>", self.start, s);
    }
}