use iron::{Request, Response, IronResult, status};
use iron::headers;
use router::Router;
use model;
use plugin::Pluggable;
use model::{AppDb};
use persistent::Read;

fn get(req: &mut Request) -> IronResult<Response> {
    let pool = req.get::<Read<AppDb>>().unwrap();
    let conn = pool.get().unwrap();
    let id = req.extensions.get::<Router>()
        .unwrap()
        .find("id")
        .and_then(|id| id.parse::<i32>().ok());


    let id = match id {
        Some(id) => id,
        None => return Ok(Response::with((status::BadRequest, "Bad Request")))
    };


    let usr = model::models::Usr::find(&conn, id);

    match usr {
        Some(u) => Ok(Response::with((status::Ok, u))),
        None => Ok(Response::with((status::NotFound, "Not Found")))
    }
}

fn post(req: &mut Request) -> IronResult<Response> {
    let pool = req.get::<Read<AppDb>>().unwrap();
    let conn = pool.get().unwrap();
    let h = req.headers.get::<headers::Authorization<headers::Basic>>();

    let r = h.map(|auth| {
        let n = &auth.username;
        let p = &auth.password.clone().unwrap();
        model::save_usr_secure(&conn, n, p)
    });

    match r {
        Some(_) => Ok(Response::with(status::Ok)),
        None => Ok(Response::with((status::Unauthorized, "Not Authorized")))
    }
}

pub fn index() -> Router {
    let router = router!(index: get "/" => get,
                         query: get "/:id" => get,
                         index: post "/" => post);
    router
}