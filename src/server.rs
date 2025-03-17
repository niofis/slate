use std::sync::mpsc::{channel, Sender};

use actix_web::{
    web::{self},
    App, HttpRequest, HttpResponse, HttpServer, Responder,
};

use crate::types::{GetContentMessage, UrlPath, WebResponse};

pub async fn start(sender: Sender<GetContentMessage>) -> std::io::Result<()> {
    println!("Server started on 127.0.0.1:8080");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(sender.clone()))
            .default_service(web::to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn index(ctx: web::Data<Sender<GetContentMessage>>, req: HttpRequest) -> impl Responder {
    let route = req.path().to_string();
    let (tx, rx) = channel();
    ctx.send(GetContentMessage(UrlPath(route), tx.clone()))
        .unwrap();
    let response = rx.recv().unwrap();
    match response {
        WebResponse::NotFound => HttpResponse::NotFound().finish(),
        WebResponse::Redirect(url) => HttpResponse::PermanentRedirect()
            .append_header(("Location", url))
            .finish(),
        WebResponse::Content(content) => match content {
            crate::types::WebContent::Html(html) => HttpResponse::Ok()
                .append_header(("Content-Type", "text/html"))
                .body(html),
            crate::types::WebContent::Css(css) => HttpResponse::Ok()
                .append_header(("Content-Type", "text/css"))
                .body(css),
            crate::types::WebContent::JavaScript(js) => HttpResponse::Ok()
                .append_header(("Content-Type", "application/javascript"))
                .body(js),
            crate::types::WebContent::Jpeg(jpeg) => HttpResponse::Ok()
                .append_header(("Content-Type", "image/jpeg"))
                .body(jpeg),
            crate::types::WebContent::Png(png) => HttpResponse::Ok()
                .append_header(("Content-Type", "image/png"))
                .body(png),
        },
    }
}
