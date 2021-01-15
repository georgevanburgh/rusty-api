mod oauth;
mod session_store;
mod app;
mod github;
mod env;

use actix_web::{App, HttpServer, middleware::Logger};
use oauth::OauthAccessor;
use session_store::SessionStore;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let accessor = SessionStore::new();
    let auth = OauthAccessor::new();

    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .data(accessor.clone())
            .data(auth.clone())
            .service(app::hello)
            .service(app::create_session)
            .service(app::has_session)
            .service(app::debug)
            .wrap(Logger::default())
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}
