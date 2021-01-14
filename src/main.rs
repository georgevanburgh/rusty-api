mod session_store;
mod oauth;

use oauth::OauthAccessor;
use session_store::{SafeSessionStore, Session, SharedSessionStore, new_session_store};
use serde::{Deserialize};

use actix_web::{App, HttpMessage, HttpRequest, HttpResponse, HttpServer, Responder, Result, cookie::Cookie, error, get, post, web};
use actix_web::{middleware::Logger};
use env_logger::Env;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!\n")
}

#[get("/has_session")]
async fn has_session(accessor: web::Data<SharedSessionStore>, req: HttpRequest) -> Result<&'static str> {
    let session_id = req.cookie("session_id");
    let accessor = SafeSessionStore::new(accessor.get_ref().clone());
    let session = session_id.and_then(|s| accessor.get_session(s.value()));

    match session {
        Some(_) => Ok("has_session"),
        None => Err(error::ErrorUnauthorized("no session id"))
    }
}

#[derive(Deserialize)]
struct Code {
    code: String
}

#[post("/create_session")]
async fn create_session(accessor: web::Data<SharedSessionStore>, auth: web::Data<OauthAccessor>, info: web::Json<Code>) -> Result<HttpResponse> {
    let mut accessor = SafeSessionStore::new(accessor.get_ref().clone());

    let token = auth.get_token(&info.code).await;

    println!("token: {:?}", token);

    let s = Session::new();
    let new_session_id = accessor.create_session(s);
    let session_cookie = Cookie::build("session_id", new_session_id).finish();
    Ok(HttpResponse::Ok().cookie(session_cookie).body("new session"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let accessor = new_session_store();
    let auth = OauthAccessor::new();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .data(accessor.clone())
            .data(auth.clone())
            .service(hello)
            .service(create_session)
            .service(has_session)
            .wrap(Logger::default())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
