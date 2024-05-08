use actix::{Actor, StreamHandler};
use actix_web::{
    guard::{Delete, Get, Post, Put},
    web, App, Error, HttpRequest, HttpResponse, HttpServer,
};
use actix_web_actors::ws::{self, ProtocolError};
use default_env::default_env;
use std::convert::{TryFrom, TryInto};
use std::ops::RangeInclusive;
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

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BackendError {
    NormalClosure = 1000,
    GoingAway,
    ProtocolError,
    UnsupportedData,
    NoStatusReceived,
    AbnormalClosure,
    InvalidFramePayloadData,
    PolicyViolation,
    MessageTooBig,
    MandatoryExt,
    InternalError,
    ServiceRestart,
    TryAgainLater,
    BadGateway,
    TlsHandshake,
    Unauthorized = 3000,
    ReAuth = 4000,
    Banned,
    TooManyConnections,
}
impl TryFrom<u16> for BackendError {
    type Error = ();
    fn try_from(value: u16) -> Result<Self, ()> {
        match value {
            1000 => Ok(BackendError::NormalClosure),
            1001 => Ok(BackendError::GoingAway),
            1002 => Ok(BackendError::ProtocolError),
            1003 => Ok(BackendError::UnsupportedData),
            1004 => Ok(BackendError::NoStatusReceived),
            1005 => Ok(BackendError::AbnormalClosure),
            1006 => Ok(BackendError::InvalidFramePayloadData),
            1007 => Ok(BackendError::PolicyViolation),
            1008 => Ok(BackendError::MessageTooBig),
            1009 => Ok(BackendError::MandatoryExt),
            1010 => Ok(BackendError::InternalError),
            1011 => Ok(BackendError::ServiceRestart),
            1012 => Ok(BackendError::TryAgainLater),
            1013 => Ok(BackendError::BadGateway),
            1014 => Ok(BackendError::TlsHandshake),
            3000 => Ok(BackendError::Unauthorized),
            4000 => Ok(BackendError::ReAuth),
            4001 => Ok(BackendError::Banned),
            4002 => Ok(BackendError::TooManyConnections),
            _ => Err(()),
        }
    }
}
#[cfg(test)]
#[test]
fn test_backend_error_round_trip() {
    let values = [
        BackendError::NormalClosure,
        BackendError::GoingAway,
        BackendError::ProtocolError,
        BackendError::UnsupportedData,
        BackendError::NoStatusReceived,
        BackendError::AbnormalClosure,
        BackendError::InvalidFramePayloadData,
        BackendError::PolicyViolation,
        BackendError::MessageTooBig,
        BackendError::MandatoryExt,
        BackendError::InternalError,
        BackendError::ServiceRestart,
        BackendError::TryAgainLater,
        BackendError::BadGateway,
        BackendError::TlsHandshake,
        BackendError::Unauthorized,
        BackendError::ReAuth,
        BackendError::Banned,
        BackendError::TooManyConnections,
    ];
    for v in values {
        assert_eq!(BackendError::try_from(v as u16), Ok(v))
    }
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq)]
enum C2SMessage<'a> {
    Token(&'a [u8]) = 0,
    Ping(u32, bool, &'a [u8]) = 1,
    Sub(u128) = 2, // owo
    Unsub(u128) = 3,
}
// 6 - 6
impl<'a> TryFrom<&'a [u8]> for C2SMessage<'a> {
    type Error = MessageLoadError;
    fn try_from(buf: &'a [u8]) -> Result<Self, <Self as TryFrom<&'a [u8]>>::Error> {
        if buf.len() == 0 {
            Err(MessageLoadError::BadLength("C2SMessage", 1, false, 0))
        } else {
            match buf[0] {
                0 => Ok(C2SMessage::Token(&buf[1..])),
                1 => {
                    if buf.len() >= 6 {
                        Ok(C2SMessage::Ping(
                            u32::from_be_bytes((&buf[1..5]).try_into().unwrap()),
                            buf[5] != 0,
                            &buf[6..],
                        ))
                    } else {
                        Err(MessageLoadError::BadLength(
                            "C2SMessage::Ping",
                            6,
                            false,
                            buf.len(),
                        ))
                    }
                }
                2 => {
                    if buf.len() == 17 {
                        Ok(C2SMessage::Sub(u128::from_be_bytes(
                            (&buf[1..]).try_into().unwrap(),
                        )))
                    } else {
                        Err(MessageLoadError::BadLength(
                            "C2SMessage::Sub",
                            17,
                            true,
                            buf.len(),
                        ))
                    }
                }
                3 => {
                    if buf.len() == 17 {
                        Ok(C2SMessage::Unsub(u128::from_be_bytes(
                            (&buf[1..]).try_into().unwrap(),
                        )))
                    } else {
                        Err(MessageLoadError::BadLength(
                            "C2SMessage::Unsub",
                            17,
                            true,
                            buf.len(),
                        ))
                    }
                }
                a => Err(MessageLoadError::BadEnum(
                    "C2SMessage.type",
                    0..=3,
                    a.into(),
                )),
            }
        }
    }
}
impl<'a> Into<Box<[u8]>> for C2SMessage<'a> {
    fn into(self) -> Box<[u8]> {
        use std::iter;
        let a: Box<[u8]> = match self {
            C2SMessage::Token(t) => iter::once(0).chain(t.into_iter().copied()).collect(),
            C2SMessage::Ping(p, s, d) => iter::once(1)
                .chain(p.to_be_bytes())
                .chain(iter::once(s.into()))
                .chain(d.into_iter().copied())
                .collect(),
            C2SMessage::Sub(s) => iter::once(2).chain(s.to_be_bytes()).collect(),
            C2SMessage::Unsub(s) => iter::once(3).chain(s.to_be_bytes()).collect(),
        };
        a
    }
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq)]
enum S2CMessage<'a> {
    Auth = 0,
    Ping(u128, u8, bool, &'a [u8]) = 1,
    Event(u128) = 2,
    Toast(u8, &'a str, Option<&'a str>) = 3,
    Chat(&'a str) = 4,
    Notice(u8) = 5,
}
impl<'a> TryFrom<&'a [u8]> for S2CMessage<'a> {
    type Error = MessageLoadError;
    fn try_from(_: &'a [u8]) -> Result<Self, <Self as TryFrom<&'a [u8]>>::Error> {
        todo!()
    }
}
impl<'a> Into<&'a [u8]> for S2CMessage<'a> {
    fn into(self) -> &'a [u8] {
        todo!()
    }
}

enum MessageLoadError {
    BadEnum(&'static str, RangeInclusive<usize>, usize),
    BadLength(&'static str, usize, bool, usize),
}
use std::fmt;
impl fmt::Display for MessageLoadError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::BadEnum(f, r, c) => write!(
                fmt,
                "value {f} must be {} to {} inclusive, got {c}",
                r.start(),
                r.end()
            ),
            Self::BadLength(f, n, e, c) => write!(
                fmt,
                "buffer wrong size for {f} — must be {} {n} bytes, got c",
                if *e { "exactly" } else { "at least" }
            ),
        }
    }
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
