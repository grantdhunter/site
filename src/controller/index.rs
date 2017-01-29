use iron::{Request, Response, IronResult, status};
use router::Router;

fn get(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Index here")))
}

pub fn index() -> Router {
    let router = router!(index: get "/" => get);
    router
}