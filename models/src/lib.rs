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

extern crate iron;
extern crate base64;
extern crate rocket;

use diesel::pg::PgConnection;
use diesel::Connection;

use r2d2_diesel::ConnectionManager;

use std::error::Error;
use std::fmt;
use crypto::bcrypt_pbkdf;



pub mod schema;
pub mod usr;
pub mod usr_secure;

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




#[derive(Debug)]
pub struct AuthError;

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt("Authentication Error", f)
    }
}

impl Error for AuthError {
    fn description(&self) -> &str {
        "Authentication Error"
    }
}

fn hash_password(password: &str, salt: &str) -> String {
    let mut out = [0u8; 32];
    bcrypt_pbkdf::bcrypt_pbkdf(password.as_bytes(), salt.as_bytes(), 32, &mut out);

    let mut password_hash = String::with_capacity(out.len() * 2);
    for c in &out {
        password_hash.push_str(&format!("{:02x}", c));
    }
    password_hash
}
