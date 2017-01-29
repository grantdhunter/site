use mount::Mount;
use iron::{Chain};
use bodyparser;
use persistent::Read;

use controller;
use middleware;
use model::{create_connection_pool, AppDb};

const MAX_BODY_LENGTH: usize = 1024 * 1024 * 10;

pub fn router() -> Chain {
    let pool = create_connection_pool();
    let mut root = Mount::new();

    root.mount("/", controller::index::index());
    root.mount("/usr", controller::usr::index());

    let mut chain = Chain::new(root);

    //Before
    chain.link_before(Read::<bodyparser::MaxBodyLength>::one(MAX_BODY_LENGTH));
    chain.link_before(Read::<AppDb>::one(pool));
    chain.link_before(middleware::auth::AuthenticationMiddleware);
    //After
    chain.link_after(middleware::not_found::NotFound404);
    chain.link_after(middleware::auth::AuthenticationMiddleware);

    chain
}