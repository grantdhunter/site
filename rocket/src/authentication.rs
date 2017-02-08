use base64::decode;
use rocket::request::{self, Request, FromRequest};
use rocket::Outcome::Failure;
use std::str;

use models::usr_secure::UsrSecure;


pub struct Authentication(UsrSecure);


impl<'a, 'r> FromRequest<'a, 'r> for Authentication {
    type Error = ();
    fn from_request(req: &'a Request<'r>) -> request::Outcome<Self, ()> {
        let conn = super::DB_POOL.get().unwrap();

        let h = req.headers().get("Authorization").next().unwrap_or_default();

        let r = decode(&h).map(|s| str::from_utf8(&s).unwrap().split(':').collect());

        match r {
            Ok(r) => {
               UsrSecure::find(&conn, r.0)
            }
            Err(err) => Failure((err))
        }
    }
}