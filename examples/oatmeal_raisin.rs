extern crate iron;
extern crate oatmeal_raisin;
extern crate persistent;

use persistent::Read;
use iron::prelude::*;
use iron::status;
use oatmeal_raisin::{Cookie, CookieJar, SetCookie, SigningKey};

fn handle(req: &mut Request) -> IronResult<Response> {
    let cookie_jar = req.get_mut::<CookieJar>().unwrap();
    cookie_jar.signed().add(Cookie::new("favorite".into(), "oatmeal_raisin".into()));
    Ok(Response::with(status::Ok))
}

fn main() {
    let mut chain = Chain::new(handle);
    chain.link_before(Read::<SigningKey>::one(b"f8f9eaf1ecdedff5e5b749c58115441e"));
    chain.link_after(SetCookie);
    Iron::new(chain).http("localhost:3000").unwrap();
}
