use crate::traits::{HTML};

#[derive(Debug)]
pub struct RawHTML{
    pub text: String
}

impl HTML for RawHTML {
    fn to_html(&self) -> String {
        return format!("{}", self.text);
    }
}
