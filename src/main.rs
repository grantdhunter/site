#![feature(custom_derive)]
#![feature(custom_attribute)]

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate rustc_serialize;
extern crate toml;
extern crate iron;
#[macro_use]
extern crate router;
extern crate mount;
extern crate bodyparser;
extern crate persistent;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate params;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate plugin;
extern crate crypto;
extern crate rand;



mod config;
mod routes;
mod controller;
mod model;
mod middleware;

use iron::prelude::*;


fn main() {
    let config = config::get();
    let _server = Iron::new(routes::router()).http(config.ip_and_port().as_str()).unwrap();
    println!("Listening on {}", config.ip_and_port());
}
