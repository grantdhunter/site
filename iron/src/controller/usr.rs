use iron::{Request, Response, IronResult, status};
use iron::headers;
use iron::Chain;
use middleware;
use router::Router;
use models;
use plugin::Pluggable;
use persistent::Read;
use routes::AppDb;

use iron::modifier::Modifier;

use serde_json;

impl Modifier<Response> for models::models::Usr {
    fn modify(self, res: &mut Response) {
        let _ = serde_json::to_string(&self).map(|s| s.into_bytes().modify(res));
    }
}

fn get(req: &mut Request) -> IronResult<Response> {
    let pool = req.get::<Read<AppDb>>().unwrap();
    let conn = pool.get().unwrap();
    let id = req.extensions
        .get::<Router>()
        .unwrap()
        .find("id")
        .and_then(|id| id.parse::<i32>().ok());


    let id = match id {
        Some(id) => id,
        None => return Ok(Response::with((status::BadRequest, "Bad Request"))),
    };


    let usr = models::models::Usr::find(&conn, id);

    match usr {
        Some(u) => Ok(Response::with((status::Ok, u))),
        None => Ok(Response::with((status::NotFound, "Not Found"))),
    }
}

fn post(req: &mut Request) -> IronResult<Response> {
    let pool = req.get::<Read<AppDb>>().unwrap();
    let conn = pool.get().unwrap();
    let h = req.headers.get::<headers::Authorization<headers::Basic>>();

    let r = h.map(|auth| {
        let n = &auth.username;
        let p = &auth.password.clone().unwrap();
        let u = models::models::NewUsr::new()
            .email(n.clone())
            .save(&conn);
        models::models::NewUsrSecure::new(u.id, n, p).save(&conn)
    });

    match r {
        Some(_) => Ok(Response::with(status::Ok)),
        None => Ok(Response::with((status::Unauthorized, "Not Authorized"))),
    }
}

pub fn index() -> Chain {
    let mut get_chain = Chain::new(get);
    get_chain.link_before(middleware::auth::AuthenticationMiddleware);

    let mut query_chain = Chain::new(get);
    query_chain.link_before(middleware::auth::AuthenticationMiddleware);

    let chain = Chain::new(router!(index: get "/" => get_chain,
                         query: get "/:id" => query_chain,
                         index: post "/" => post));


    chain
}
