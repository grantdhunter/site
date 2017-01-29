use iron::{Request, Response, IronResult, status};
use iron::Chain;
use router::Router;
use middleware;
use mount::Mount;


fn get(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Index here")))
}

pub fn index() -> Chain {
    let mut chain = Chain::new(router!(index: get "/" => get));

    chain.link_before(middleware::auth::AuthenticationMiddleware);
    chain
}
