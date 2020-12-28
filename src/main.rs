#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};

#[derive(Debug)]
enum ApiAddressError {
    Missing
}

struct IPAddress(String);

impl<'a, 'r> FromRequest<'a, 'r> for IPAddress {
    type Error = ApiAddressError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        match request.remote() {
            Some(socket) => Outcome::Success(IPAddress(socket.ip().to_string())),
            None => Outcome::Failure((Status::BadRequest, ApiAddressError::Missing))
        }
    }
}

#[get("/")]
fn index(addr: IPAddress) -> String {
    addr.0
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}