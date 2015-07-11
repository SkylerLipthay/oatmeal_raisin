//! Cookies plugin for Iron
//!
//! This plugin accepts and sets cookies through the `Cookie` and `Set-Cookie` headers.

extern crate cookie;
extern crate iron;
extern crate persistent;
extern crate plugin;

use std::ops::{Deref, DerefMut};

pub use cookie::Cookie;
use iron::{headers, IronResult, Request, Response};
use iron::middleware::AfterMiddleware;
use iron::modifier::{Modifier, Set};
use iron::typemap::Key;
use plugin::{Pluggable, Plugin};

pub struct SigningKey;

/// A variable for modifying the security key used to sign or unsign cookies.
///
/// See the `cookie` crate for details on how this key is used for signing or encrypting cookies.
impl Key for SigningKey {
    type Value = &'static [u8];
}

/// A type for managing a request's incoming and outgoing cookies.
pub struct CookieJar(cookie::CookieJar<'static>);

impl Key for CookieJar {
    type Value = CookieJar;
}

impl<'a, 'b> Plugin<Request<'a, 'b>> for CookieJar {
    type Error = ();

    fn eval(req: &mut Request) -> Result<CookieJar, ()> {
        let key = req.get::<persistent::Read<SigningKey>>().ok().map(|k| *k).unwrap_or(b"");
        match req.headers.get::<headers::Cookie>() {
            Some(header) => Ok(CookieJar(header.to_cookie_jar(key))),
            None => return Ok(CookieJar(cookie::CookieJar::new(key))),
        }
    }
}

impl Deref for CookieJar {
    type Target = cookie::CookieJar<'static>;

    fn deref(&self) -> &cookie::CookieJar<'static> {
        &self.0
    }
}

impl DerefMut for CookieJar {
    fn deref_mut(&mut self) -> &mut cookie::CookieJar<'static> {
        &mut self.0
    }
}

impl<'a> Modifier<Response> for &'a CookieJar {
    fn modify(self, res: &mut Response) {
        res.headers.set(headers::SetCookie::from_cookie_jar(self))
    }
}

/// A helper middleware to always apply changes from the request's `CookieJar` to the response's
/// `Set-Cookie` header.
pub struct SetCookie;

impl AfterMiddleware for SetCookie {
    fn after(&self, req: &mut Request, res: Response) -> IronResult<Response> {
        Ok(res.set(req.get_ref::<CookieJar>().unwrap()))
    }
}
