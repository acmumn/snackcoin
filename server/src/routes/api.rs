use iron::{status, prelude::*};
use router::Router;

pub fn make_handler() -> Router {
    router! {
        index: get "/" => the_source_code_is_self_documenting,
    }
}

fn the_source_code_is_self_documenting(_req: &mut Request) -> IronResult<Response> {
    use iron::headers::Location;

    const GITHUB_LINK: &str = "https://github.com/acmumn/snackcoin/tree/master/server/src/routes/";

    let mut res = Response::with(status::SeeOther);
    res.headers.set(Location(GITHUB_LINK.to_string()));
    Ok(res)
}
