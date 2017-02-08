use iron::error::IronError;
use iron::{BeforeMiddleware, AfterMiddleware, status, Request, Response, IronResult};
use plugin::Pluggable;
use routes::AppDb;
use persistent::Read;
use iron::headers;

pub struct AuthenticationMiddleware;

use models::usr_secure::UsrSecure;
use models::AuthError;

impl BeforeMiddleware for AuthenticationMiddleware {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        let pool = req.get::<Read<AppDb>>().unwrap();
        let conn = pool.get().unwrap();
        let h = req.headers.get::<headers::Authorization<headers::Basic>>();

        let (n, p) = match h {
            Some(auth) => {
                let n = auth.username.clone();
                let p = auth.password.clone().unwrap_or_default();
                (n, p)
            }
            None => return Err(IronError::new(AuthError, status::Unauthorized)),
        };
        match UsrSecure::check_password(&conn, &n, &p) {
            Ok(_) => Ok(()),
            Err(err) => Err(IronError::new(err, status::Unauthorized)),
        }
    }
}


impl AfterMiddleware for AuthenticationMiddleware {
    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response> {
        if err.error.downcast::<AuthError>().is_some() {
            let mut resp = Response::with((status::Unauthorized, "401 Unauthorized"));
            resp.headers.set_raw("WWW-Authenticate", vec![b"Basic".to_vec()]);
            return Ok(resp);
        }

        Err(err)
    }
}
