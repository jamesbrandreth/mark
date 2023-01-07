use crate::traits::{HTML};

#[derive(Debug)]
pub struct ThematicBreak{}


impl HTML for ThematicBreak {
    fn to_html(&self) -> String {
        return String::from("<hr />");
    }
}
