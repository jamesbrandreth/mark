use crate::traits::{HTML};

#[derive(Debug)]
pub struct Code{
    pub text: String
}

impl HTML for Code {
    fn to_html(&self) -> String {
        return format!(
                "<pre><code>{}</pre></code>",
            self.text
        );
    }
}
