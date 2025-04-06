use std::{sync::mpsc::Receiver, thread};

use crate::{
    file_system,
    templating::Template,
    types::{Configuration, GetContentMessage, UrlPath, WebResponse},
    web_content::process_content,
};

pub fn start(rx: Receiver<GetContentMessage>) {
    thread::spawn(move || loop {
        if let Ok(GetContentMessage(url_path, tx)) = rx.recv() {
            // Force clients to access path with a trailing /
            if file_system::is_dir(&url_path) && !url_path.0.ends_with("/") {
                tx.send(WebResponse::Redirect(format!("{}/", url_path.0)))
                    .unwrap();
            } else {
                let config = file_system::find_file(&url_path, "config.json")
                    .map(|file| serde_json::from_slice::<Configuration>(&file.content).unwrap());
                let template = file_system::find_file(&url_path, "template.html")
                    .map(|file| Template::new(file));
                if let Some(redirect) =
                    file_system::read_path(&UrlPath(format!("{}{}", url_path.0, "url.redirect")))
                {
                    let goto = String::from_utf8(redirect.content)
                        .unwrap()
                        .trim()
                        .to_string();
                    tx.send(WebResponse::Redirect(goto)).unwrap();
                    continue;
                }

                let file = file_system::read_path(&url_path);
                match file {
                    None => tx.send(WebResponse::NotFound).unwrap(),
                    Some(content) => tx
                        .send(WebResponse::Content(process_content(
                            template, config, content,
                        )))
                        .unwrap(),
                }
            }
        }
    });
}
