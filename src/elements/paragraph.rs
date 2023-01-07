use crate::traits::{HTML, MarkDown};

#[derive(Debug)]
pub struct Paragraph{
    pub text: String
}

impl MarkDown for Paragraph {
    fn from_markdown(s: &str) -> Paragraph {
        return Paragraph{
            text: String::from(s)
        }
    }
}

impl HTML for Paragraph {
    fn to_html(&self) -> String {
        return format!(
                "<p>{}</p>",
            self.text
        );
    }
}
