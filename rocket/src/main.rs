#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate rustc_serialize;
extern crate toml;
extern crate models;
#[macro_use]
extern crate lazy_static;
extern crate base64;

use rocket::request::{self, Request, FromRequest};
use rocket::outcome::Outcome::*;

use models::usr_secure::UsrSecure;
use models::usr::Usr;
use models::ConnectionPool;

mod config;
mod authentication;

use authentication::Authentication;

lazy_static! {
    pub static ref DB_POOL:ConnectionPool = models::create_connection_pool(&config::get()
                                                    .db_connection());
}

#[post("/")]
fn login(auth: Authentication) -> &'static str {
    "Login"
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}


fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/usr/login", routes![login])
        .launch();
}
