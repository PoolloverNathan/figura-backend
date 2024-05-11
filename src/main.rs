mod backend;
mod backend_error;
mod message;

use actix::{Actor, StreamHandler};
use actix_web::{
    guard::{Delete, Get, Post, Put},
    web::{self, Path},
    App, Error, HttpRequest, HttpResponse, HttpServer,
};
use actix_web_actors::ws::{self, ProtocolError};
use std::fs::File;
use std::io::BufReader;

struct Ws;
impl Ws {
    async fn start(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
        ws::start(Ws, &req, stream)
    }
}
impl StreamHandler<Result<actix_web_actors::ws::Message, ProtocolError>> for Ws {
    fn handle(
        &mut self,
        item: Result<actix_web_actors::ws::Message, ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        let _ = (item, ctx);
        todo!()
    }
}
impl Actor for Ws {
    type Context = ws::WebsocketContext<Self>;
}

fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/api/").to(|| async { ":3" }));
    cfg.service(
        web::resource("/api/limits")
            .guard(Get())
            .to(|| async move { todo!("@toomanylimits") as &str }),
    );
    cfg.service(
        web::resource("/api/version")
            .guard(Get())
            .to(|| async move { "{\"release\":\"2.7.1\",\"prerelease\":\"2.7.1\"}" }),
    );
    cfg.service(
        web::resource("/api/motd")
            .guard(Get())
            .to(|| async move { "\"did a coding thing :)\"" }),
    );
    cfg.service(
        web::resource("/api/equip")
            .guard(Post())
            .to(|| async move { todo!("equip avatar") as &str }),
    );
    cfg.service(
        web::resource("/api/{user}")
            .guard(Get())
            .to(|p: Path<(String,)>| async move { todo!("get user {}", p.0) as &str }),
    );
    cfg.service(
        web::resource("/api/{avatar}")
            .guard(Put())
            .to(|p: Path<(String,)>| async move { todo!("put avatar {}", p.0) as &str }),
    );
    cfg.service(
        web::resource("/api/{avatar}")
            .guard(Delete())
            .to(|p: Path<(String,)>| async move { todo!("delete avatar {}", p.0) as &str }),
    );
    cfg.route("/ws", web::get().to(Ws::start));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let tls_config = rustls::ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(
            rustls_pemfile::certs(&mut BufReader::new(File::open("cert.pem")?))
                .flatten()
                .map(|a| rustls::Certificate(a.into_iter().copied().collect()))
                .collect(),
            rustls_pemfile::private_key(&mut BufReader::new(File::open("key.pem")?))
                .into_iter()
                .flatten()
                .map(|a| rustls::PrivateKey(a.secret_der().into_iter().copied().collect()))
                .next()
                .unwrap(),
        )
        .unwrap();
    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .configure(config)
    })
    .bind_rustls("0.0.0.0:5443", tls_config)?
    .run()
    .await
}
