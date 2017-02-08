use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use super::schema::usr_secure;
use rand::Rng;
use rand::thread_rng;

use AuthError;
use hash_password;

#[derive(Debug, Insertable)]
#[table_name = "usr_secure"]
pub struct NewUsrSecure {
    pub id: i32,
    pub username: String,
    password: String,
    salt: String,
}


#[derive(Debug, Queryable, Associations, Identifiable)]
#[table_name = "usr_secure"]
#[belongs_to(usr)]
pub struct UsrSecure {
    pub id: i32,
    pub username: String,
    password: String,
    salt: String,
}


impl NewUsrSecure {
    pub fn new(id: i32, username: &str, password: &str) -> Self {
        let salt = thread_rng().gen_ascii_chars().take(64).collect::<String>();
        let hashed_password = hash_password(password, &salt);

        NewUsrSecure {
            id: id,
            username: username.to_string(),
            password: hashed_password,
            salt: salt,
        }
    }

    pub fn save(&self, conn: &PgConnection) -> UsrSecure {
        diesel::insert(self)
            .into(usr_secure::table)
            .get_result(conn)
            .expect("Something went wrong")
    }
}

impl UsrSecure {
    pub fn find(conn: &PgConnection, name: &str) -> Option<UsrSecure> {
        use super::schema::usr_secure::dsl::*;

        usr_secure.filter(username.eq(name))
            .load::<UsrSecure>(conn)
            .ok()
            .and_then(|mut u| u.pop())
    }

    pub fn check_password(conn: &PgConnection,
                          name: &str,
                          password: &str)
                          -> Result<(), AuthError> {
        match UsrSecure::find(conn, name) {
            Some(u) => {
                if &u.password == &u.hash_password(password) {
                    return Ok(());
                }
                Err(AuthError)
            }
            None => Err(AuthError),
        }
    }

    pub fn hash_password(&self, password: &str) -> String {
        hash_password(password, &self.salt)
    }
}