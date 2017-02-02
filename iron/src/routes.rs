use mount::Mount;
use iron::Chain;
use bodyparser;
use persistent::Read;

use controller;
use middleware;
use models::{create_connection_pool, ConnectionPool};
use iron::typemap::Key;

use config;

pub struct AppDb;

impl Key for AppDb {
    type Value = ConnectionPool;
}


const MAX_BODY_LENGTH: usize = 1024 * 1024 * 10;

pub fn router() -> Chain {

    let config = config::get();
    let pool = create_connection_pool(&config.db_connection());
    let mut root = Mount::new();

    root.mount("/", controller::index::index());
    root.mount("/usr", controller::usr::index());

    let mut chain = Chain::new(root);

    //Before
    chain.link_before(Read::<bodyparser::MaxBodyLength>::one(MAX_BODY_LENGTH));
    chain.link_before(Read::<AppDb>::one(pool));
    //After
    chain.link_after(middleware::not_found::NotFound404);
    chain.link_after(middleware::auth::AuthenticationMiddleware);

    chain
}
