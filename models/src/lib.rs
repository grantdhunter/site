#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_codegen;

extern crate r2d2;
extern crate r2d2_diesel;
extern crate crypto;
extern crate rand;

use diesel::pg::PgConnection;
use diesel::Connection;

use r2d2_diesel::ConnectionManager;


pub mod schema;
pub mod models;

pub type ConnectionPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[allow(dead_code)]
pub fn establish_connection(url: &str) -> PgConnection {
    PgConnection::establish(url).expect(&format!("Error connecting to {}", url))
}

pub fn create_connection_pool(url: &str) -> ConnectionPool {
    let config = r2d2::Config::default();
    let manager = ConnectionManager::<PgConnection>::new(url);
    r2d2::Pool::new(config, manager).expect("Failed to create pool.")
}
