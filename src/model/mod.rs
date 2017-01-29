use diesel::pg::PgConnection;
use diesel::Connection;
use diesel;
use diesel::prelude::*;
use config;
use iron::typemap::Key;

use r2d2_diesel::ConnectionManager;
use r2d2;


pub mod schema;
pub mod models;

use self::schema::*;
use self::models::*;

pub type ConnectionPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct AppDb;

impl Key for AppDb {
    type Value = ConnectionPool;
}

#[allow(dead_code)]
pub fn establish_connection() -> PgConnection {
    let database_url = config::get().db_connection();
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn create_connection_pool() -> ConnectionPool {
    let config = r2d2::Config::default();
    let database_url = config::get().db_connection();
    let manager = ConnectionManager::<PgConnection>::new(database_url.as_str());
    r2d2::Pool::new(config, manager).expect("Failed to create pool.")
}


pub fn save_usr_secure(conn: &PgConnection, name: &str, password: &str) -> UsrSecure {
    let email = name.to_string();
    let usr = NewUsr::new()
        .email(email)
        .finalize();
    let u: Usr = diesel::insert(&usr)
        .into(usr::table)
        .get_result(conn)
        .expect("Something went wrong");

    let usr = NewUsrSecure::new(u.id, name, password);


    diesel::insert(&usr)
        .into(usr_secure::table)
        .get_result(conn)
        .expect("Something went wrong")
}
