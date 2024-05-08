mod backend_error;
mod message;

use actix::{Actor, StreamHandler};
use actix_web::{
    guard::{Delete, Get, Post, Put},
    web, App, Error, HttpRequest, HttpResponse, HttpServer,
};
use actix_web_actors::ws::{self, ProtocolError};
use default_env::default_env;
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
    cfg.service(web::resource("/").to(|| async { ":3" }));
    cfg.service(
        web::resource("/limits")
            .guard(Get())
            .to(|| async { "@toomanylimits" }),
    );
    cfg.service(
        web::resource("/version")
            .guard(Get())
            .to(|| async { default_env!("version", "dev") }),
    );
    cfg.service(
        web::resource("/motd")
            .guard(Get())
            .to(|| async { "did a coding thing :)" }),
    );
    cfg.service(
        web::resource("/equip")
            .guard(Post())
            .to(|| async { "equup avatar" }),
    );
    cfg.service(
        web::resource("/{user}")
            .guard(Get())
            .to(|| async { "user" }),
    );
    cfg.service(
        web::resource("/{avatar}")
            .guard(Put())
            .to(|| async { "put avatar" }),
    );
    cfg.service(
        web::resource("/{avatar}")
            .guard(Delete())
            .to(|| async { "delete avatar" }),
    );
    cfg.route("/ws", web::get().to(Ws::start));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(config))
        .bind("0.0.0.0:25565")?
        .run()
        .await
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use actix_web::dev::Service;
//     use actix_web::{http, test, App, Error};

//     #[actix_rt::test]
//     async fn test() -> Result<(), Error> {
//         let mut app = test::init_service(App::new().configure(config)).await;

//         let resp = app
//             .call(test::TestRequest::get().uri("/").to_request())
//             .await
//             .unwrap();

//         assert_eq!(resp.status(), http::StatusCode::OK);

//         let body = match resp.response().body().as_ref() {
//             Some(actix_web::body::Body::Bytes(bytes)) => bytes,
//             _ => panic!("Response error"),
//         };

//         assert_eq!(body, "Hello Nixers!\n");

//         Ok(())
//     }
// }
