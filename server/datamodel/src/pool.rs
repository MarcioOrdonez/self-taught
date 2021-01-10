pub extern crate diesel;
pub extern crate r2d2_diesel;
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use std::env;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn new_pool() -> Pool {
    dotenv().ok();
    let connspec = env::var("DATABASE_URL").expect("DATABASE_URL needed");
    let manager = ConnectionManager::<PgConnection>::new(connspec);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to build database pool.")
}
