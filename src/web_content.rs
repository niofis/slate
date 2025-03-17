use crate::{
    templating::Template,
    types::{Configuration, FileContent, HtmlString, WebContent},
};

pub fn process_content(
    template: Option<Template>,
    _config: Option<Configuration>,
    file_content: FileContent,
) -> WebContent {
    match file_content.extension.as_str() {
        "html" => WebContent::Html(file_content.content),
        "md" => {
            let md = process_markdown(file_content.content);
            let content = if let Some(mut template) = template {
                template.set_section(
                    HtmlString::from(file_content.name),
                    String::from_utf8(md).unwrap(),
                );
                template.content.into_bytes()
            } else {
                md
            };
            WebContent::Html(content)
        }
        "js" => WebContent::JavaScript(file_content.content),
        "css" => WebContent::Css(file_content.content),
        "jpeg" => WebContent::Jpeg(file_content.content),
        "png" => WebContent::Png(file_content.content),
        _ => WebContent::Html(String::from("unsuported").into_bytes()),
    }
}

fn process_markdown(md: Vec<u8>) -> Vec<u8> {
    let md_str = String::from_utf8(md).unwrap();
    let html = markdown::to_html(&md_str);
    html.into_bytes()
}
