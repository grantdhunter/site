#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate models;





#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
