use actix_web::{middleware, App, HttpServer};
use datamodel::*;
use std::io;
mod routes;
use routes::{
    subjects::{
        create_subject
}
};

#[actix_web::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let pool = new_pool();
    let bind = "127.0.0.1:8080";
    println!("Starting server at: {}", &bind);
    let app = move || {
        App::new()
            .data(pool.clone())
            .wrap(
                actix_cors::Cors::new()
                    .supports_credentials()
                    // Chromium < v76 caps at 600 seconds.
                    .max_age(600)
                    .finish(),
            )
            .wrap(middleware::Logger::default())
            .service(create_subject)
    };
    HttpServer::new(app).bind(&bind)?.run().await
}