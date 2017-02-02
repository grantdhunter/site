#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate models;

use rocket::request::{self, Request, FromRequest};
use rocket::outcome::Outcome::*;
use models::models::{UsrSecure};


#[derive(Debug)]
struct Authentication(());

impl<'a, 'r> FromRequest<'a, 'r> for Authentication {
    type Error = ();
    fn from_request(req: &'a Request<'r>) -> request::Outcome<Self, ()> {
        let h = req.headers().get("Authorization").next();
        println!("{:?}", h);
        Success(Authentication(()))
    }
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
        .mount("/login", routes![login])
        .launch();
}
