use crate::types::{FileContent, HtmlString};

pub struct Template {
    pub content: HtmlString,
}

impl Template {
    pub fn new(file: FileContent) -> Template {
        Template {
            content: String::from_utf8(file.content).unwrap(),
        }
    }
    pub fn set_section(&mut self, name: HtmlString, content: HtmlString) {
        let slot_id = format!("<!-- {} -->", name);
        self.content = self.content.replace(&slot_id, &content);
    }
}
