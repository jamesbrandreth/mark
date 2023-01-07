use crate::traits::HTML;
use crate::blocks::containers::list_item::ListItem;


pub struct UnorderedList{
    pub children: Vec<ListItem>,
}

impl HTML for UnorderedList {
    fn to_html(&self) -> String {
        let mut s: String = String::from("");
        for el in &self.children {
            s = s + &el.to_html();
        }
        return format!("<ul>{}</ul>", s);
    }
}