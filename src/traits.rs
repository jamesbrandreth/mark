pub trait HTML {
    fn to_html(&self) -> String;
}

pub trait MarkDown {
    fn from_markdown(s: &str) -> Self;
}
