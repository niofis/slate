use std::sync::mpsc::{channel, Sender};

use actix_web::{
    web::{self},
    App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use log::info;

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
    let method = req.method().to_string();
    let client_ip = req.connection_info().realip_remote_addr()
        .unwrap_or("unknown")
        .to_string();

    let (tx, rx) = channel();
    ctx.send(GetContentMessage(UrlPath(route.clone()), tx.clone()))
        .unwrap();
    let response = rx.recv().unwrap();
    
    let (http_response, status_code) = match response {
        WebResponse::NotFound => {
            let resp = HttpResponse::NotFound().finish();
            (resp, 404)
        },
        WebResponse::Redirect(url) => {
            let resp = HttpResponse::TemporaryRedirect()
                .append_header(("Location", url))
                .finish();
            (resp, 307)
        },
        WebResponse::Content(content) => {
            let resp = match content {
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
                WebContent::Woff2(woff2) => HttpResponse::Ok()
                    .append_header(("Content-Type", "font/woff2"))
                    .body(woff2),
            };
            (resp, 200)
        },
    };

    info!("{} - {} {} {}", client_ip, method, route, status_code);
    
    http_response
}
