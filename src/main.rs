use actix::{Actor, StreamHandler};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer, Route};
use actix_web_actors::ws::{self, ProtocolError};

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
    cfg.service(web::resource("/").to(|| async { "Hello Nixers!\n" }));
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
