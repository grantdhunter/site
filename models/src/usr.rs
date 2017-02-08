use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use super::schema::usr;


#[derive(Debug, Insertable, Clone, Default)]
#[table_name = "usr"]
pub struct NewUsr {
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub account_id: Option<i32>,
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


impl NewUsr {
    pub fn new() -> Self {
        NewUsr {
            email: None,
            first_name: None,
            last_name: None,
            account_id: None,
        }
    }

    pub fn email(&mut self, email: String) -> &mut Self {
        self.email = Some(email);
        self
    }
    pub fn first_name(&mut self, first_name: String) -> &mut Self {
        self.first_name = Some(first_name);
        self
    }

    pub fn last_name(&mut self, last_name: String) -> &mut Self {
        self.last_name = Some(last_name);
        self
    }

    pub fn account(&mut self, id: i32) -> &mut Self {
        self.account_id = Some(id);
        self
    }

    pub fn finalize(&self) -> Self {
        NewUsr { ..self.clone() }
    }

    pub fn save(&self, conn: &PgConnection) -> Usr {
        diesel::insert(self)
            .into(usr::table)
            .get_result(conn)
            .expect("Something went wrong")
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

#[cfg(feature = "iron")]
use iron::Response;
#[cfg(feature = "iron")]
use iron::modifier::Modifier;
#[cfg(feature = "iron")]
use serde_json;

#[cfg(feature = "iron")]
impl Modifier<Response> for Usr {
    fn modify(self, res: &mut Response) {
        let _ = serde_json::to_string(&self).map(|s| s.into_bytes().modify(res));
    }
}



