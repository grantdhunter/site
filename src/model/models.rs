use diesel::prelude::*;
use diesel::pg::PgConnection;
use super::schema::{usr_secure, usr};
use iron::modifier::Modifier;
use iron::Response;
use serde_json;
use std::error::Error;
use std::fmt;
use crypto::bcrypt_pbkdf;
use rand::Rng;
use rand::thread_rng;

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

pub fn hash_password(password: &str, salt: &str) -> String {
    let mut out = [0u8; 32];
    bcrypt_pbkdf::bcrypt_pbkdf(password.as_bytes(), salt.as_bytes(), 32, &mut out);

    let mut password_hash = String::with_capacity(out.len() * 2);
    for c in &out {
        password_hash.push_str(&format!("{:02x}", c));
    }
    password_hash
}

#[derive(Debug, Queryable, Associations, Identifiable)]
#[table_name = "usr_secure"]
#[belongs_to(usr)]
pub struct UsrSecure {
    pub id: i32,
    pub username: String,
    password: String,
    salt: String
}

#[derive(Debug, Insertable)]
#[table_name = "usr_secure"]
pub struct NewUsrSecure {
    pub id: i32,
    pub username: String,
    password: String,
    salt: String,
}

#[derive(Debug, Queryable, Serialize, Deserialize, Associations, Identifiable)]
#[table_name = "usr"]
pub struct Usr {
    pub id: i32,
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub account: Option<i32>,
}


#[derive(Debug, Insertable, Clone, Default)]
#[table_name = "usr"]
pub struct NewUsr {
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub account_id: Option<i32>,
}


impl UsrSecure {
    pub fn find(conn: &PgConnection, name: &str) -> Option<UsrSecure> {
        use super::schema::usr_secure::dsl::*;

        usr_secure.filter(username.eq(name))
            .load::<UsrSecure>(conn)
            .ok()
            .and_then(|mut u| u.pop())
    }

    pub fn check_password(conn: &PgConnection, name: &str, password: &str) -> Result<(), AuthError> {
        match UsrSecure::find(conn, name) {
            Some(u) => {
                if &u.password == &u.hash_password(password) {
                    return Ok(())
                }
                Err(AuthError)
            },
            None => Err(AuthError)
        }
    }

    pub fn hash_password(&self, password: &str) -> String {
        hash_password(password, &self.salt)
    }
}


impl NewUsrSecure {
    pub fn new(id: i32, username: &str, password: &str) -> Self {
        let salt = thread_rng().gen_ascii_chars().take(64).collect::<String>();
        let hashed_password = hash_password(password, &salt);

        NewUsrSecure {
            id: id,
            username: username.to_string(),
            password: hashed_password,
            salt: salt
        }
    }
}


impl NewUsr {
    pub fn new() -> Self {
        NewUsr {
            email: None,
            first_name: None,
            last_name: None,
            account_id: None,
        }
    }

    pub fn email(&mut self, email: String) -> &mut NewUsr {
        self.email = Some(email);
        self
    }
    pub fn first_name(&mut self, first_name: String) -> &mut NewUsr {
        self.first_name = Some(first_name);
        self
    }

    pub fn last_name(&mut self, last_name: String) -> &mut NewUsr {
        self.last_name = Some(last_name);
        self
    }

    pub fn account(&mut self, id: i32) -> &mut NewUsr {
        self.account_id = Some(id);
        self
    }

    pub fn finalize(&self) -> Self {
        NewUsr { ..self.clone() }
    }
}

impl Usr {
    pub fn find(conn: &PgConnection, u_id: i32) -> Option<Usr> {
        use super::schema::usr::dsl::*;
        usr.filter(id.eq(u_id))
            .load::<Usr>(conn)
            .ok()
            .and_then(|mut u| u.pop())
    }
}


impl Modifier<Response> for Usr {
    fn modify(self, res: &mut Response) {
        let _ = serde_json::to_string(&self).map(|s| s.into_bytes().modify(res));
    }
}