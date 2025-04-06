use std::sync::mpsc::{channel, Sender};

use actix_web::{
    web::{self},
    App, HttpRequest, HttpResponse, HttpServer, Responder,
};

use crate::types::{GetContentMessage, UrlPath, WebContent, WebResponse};

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
        WebResponse::Redirect(url) => HttpResponse::TemporaryRedirect()
            .append_header(("Location", url))
            .finish(),
        WebResponse::Content(content) => match content {
            WebContent::Html(html) => HttpResponse::Ok()
                .append_header(("Content-Type", "text/html"))
                .append_header(("Cross-Origin-Opener-Policy", "same-origin"))
                .append_header(("Cross-Origin-Embedder-Policy", "require-corp"))
                .body(html),
            WebContent::Css(css) => HttpResponse::Ok()
                .append_header(("Content-Type", "text/css"))
                .body(css),
            WebContent::JavaScript(js) => HttpResponse::Ok()
                .append_header(("Content-Type", "application/javascript"))
                .body(js),
            WebContent::Jpeg(jpeg) => HttpResponse::Ok()
                .append_header(("Content-Type", "image/jpeg"))
                .body(jpeg),
            WebContent::Png(png) => HttpResponse::Ok()
                .append_header(("Content-Type", "image/png"))
                .body(png),
            WebContent::Wasm(wasm) => HttpResponse::Ok()
                .append_header(("Content-Type", "application/wasm"))
                .body(wasm),
            WebContent::Ico(ico) => HttpResponse::Ok()
                .append_header(("Content-Type", "image/ico"))
                .body(ico),
            WebContent::Svg(svg) => HttpResponse::Ok()
                .append_header(("Content-Type", "image/svg"))
                .body(svg),
        },
    }
}
