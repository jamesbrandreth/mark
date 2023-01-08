use crate::traits::{HTML};

#[derive(Debug)]
pub struct Link{
    pub label: String,
    pub destination: String,
}

impl HTML for Link {
    fn to_html(&self) -> String {
        return format!(
            "<p><a href=\"{}\">{}</a></p>",
            self.destination,
            self.label
        );
    }
}
