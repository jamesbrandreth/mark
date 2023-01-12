use std::rc::Rc;

use crate::traits::{HTML};

pub struct Paragraph{
    pub text: String,
    pub parent: Rc<Box<dyn HTML>>,
}

impl Paragraph {
    pub const OPEN_RULE: &str = r"^\w+";
    pub const CLOSE_RULE: &str = r"^\w+";
}

impl HTML for Paragraph {
    fn to_html(&self) -> String {
        return format!(
                "<p>{}</p>",
            self.text
        );
    }
}
