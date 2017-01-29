use iron::error::IronError;
use iron::{AfterMiddleware, status, Request, Response, IronResult};
use router::{NoRoute};

pub struct NotFound404;

impl AfterMiddleware for NotFound404 {
    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response> {
        if err.error.downcast::<NoRoute>().is_some() {
            return Ok(Response::with((status::NotFound, "404 Not Found")))
        }

        Err(err)
    }
}