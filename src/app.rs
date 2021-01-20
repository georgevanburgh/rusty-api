use actix_web::cookie::{Cookie, SameSite};
use actix_web::{error, get, post, web, HttpMessage, HttpRequest, HttpResponse, Result};
use serde::Deserialize;

use crate::session_store::{Session, SessionStore};
use crate::{github, oauth::OauthAccessor};

#[derive(Deserialize)]
struct Code {
    code: String,
}

#[get("/")]
pub async fn hello() -> &'static str {
    "Hello world!\n"
}

#[get("/has_session")]
pub async fn has_session(
    accessor: web::Data<SessionStore>,
    req: HttpRequest,
) -> Result<&'static str> {
    let session = req
        .cookie("session_id")
        .and_then(|s| accessor.get_session(s.value()))
        .ok_or(error::ErrorUnauthorized("no session id"))?;

    log::info!("Loaded session: {:?}", session);

    Ok("has_session")
}

#[post("/create_session")]
pub async fn create_session(
    accessor: web::Data<SessionStore>,
    auth: web::Data<OauthAccessor>,
    info: web::Json<Code>,
) -> Result<HttpResponse> {
    let (token, refresh_token) = auth.get_token(&info.code).await.unwrap();

    println!("token: {}", token);
    let user_email = github::get_email(&token).await.unwrap();

    let s = Session::new(token, refresh_token, user_email);
    let new_session_id = accessor.create_session(s);
    let session_cookie = Cookie::build("session_id", new_session_id).same_site(SameSite::None).finish();
    Ok(HttpResponse::Ok()
        .cookie(session_cookie)
        .body("new session"))
}
