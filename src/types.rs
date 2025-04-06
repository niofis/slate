use serde::Deserialize;
use std::{collections::HashMap, sync::mpsc::Sender};

#[derive(Debug)]
pub struct UrlPath(pub String);

#[derive(Debug)]
pub struct FileContent {
    pub path: String,
    pub name: String,
    pub extension: String,
    pub content: Vec<u8>,
}

#[derive(Debug)]
pub enum WebResponse {
    NotFound,
    Redirect(String),
    Content(WebContent),
}

#[derive(Debug)]
pub enum WebContent {
    Html(Vec<u8>),
    Css(Vec<u8>),
    JavaScript(Vec<u8>),
    Jpeg(Vec<u8>),
    Png(Vec<u8>),
    Wasm(Vec<u8>),
    Ico(Vec<u8>),
    Svg(Vec<u8>),
}

#[derive(Debug)]
pub struct ResponseContent {
    headers: Vec<(String, String)>,
    data: Vec<u8>,
}

#[derive(Debug, Deserialize)]
pub struct Configuration {
    headers: Option<HashMap<String, String>>,
}

pub struct GetContentMessage(pub UrlPath, pub Sender<WebResponse>);

pub type HtmlString = String;
