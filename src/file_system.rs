use std::{
    ffi::OsStr,
    fs::File,
    io::Read,
    path::{absolute, Path, PathBuf},
};

use crate::types::{FileContent, UrlPath};

const PAGES_ROUTE: &str = "./pages";

pub fn is_dir(url: &UrlPath) -> bool {
    absolute_path(url).is_dir()
}

pub fn read_path(url: &UrlPath) -> Option<FileContent> {
    // If this is a folder, it will default to read
    // content.html or content.md
    let mut file_path = absolute_path(url);

    if !file_path.starts_with(absolute_path(&UrlPath(String::from("/")))) {
        return None;
    }

    if file_path.is_dir() {
        let html = file_path.join("content.html");
        let md = file_path.join("content.md");
        if html.exists() {
            file_path = html;
        } else if md.exists() {
            file_path = md;
        } else {
            return None;
        }
    }

    if file_path.exists() {
        Some(load_file(&file_path))
    } else {
        None
    }
}

pub fn find_file(url: &UrlPath, name: &str) -> Option<FileContent> {
    let stop_dir = absolute(Path::new(PAGES_ROUTE)).unwrap();
    let mut current_dir = absolute_path(url);
    while current_dir.starts_with(&stop_dir) {
        let file_name = current_dir.join(name);
        if file_name.exists() {
            return Some(load_file(&file_name));
        }
        current_dir.pop();
    }
    None
}

fn path_buf_str(path: &PathBuf) -> String {
    path.to_string_lossy().to_string()
}

fn ostr_str(path: &OsStr) -> String {
    path.to_string_lossy().to_string()
}

fn absolute_path(url: &UrlPath) -> PathBuf {
    absolute(Path::new(PAGES_ROUTE)).unwrap().join(&url.0[1..])
}

fn load_file(path: &PathBuf) -> FileContent {
    let mut content: Vec<u8> = vec![];
    _ = File::open(&path)
        .unwrap()
        .read_to_end(&mut content)
        .unwrap();
    FileContent {
        path: path_buf_str(&path),
        name: path
            .file_stem()
            .map_or_else(String::default, |e| ostr_str(&e)),
        extension: path
            .extension()
            .map_or_else(String::default, |e| ostr_str(&e)),
        content,
    }
}
