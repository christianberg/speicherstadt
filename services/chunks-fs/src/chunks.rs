extern crate iron;

use iron::prelude::*;
use iron::status;

pub struct Service {}

impl Service {
    pub fn new() -> Self {
        Iron::new(move |_: &mut Request| -> IronResult<Response> {
            Ok(Response::with((status::Ok, "Hello world!")))
        })
        .http("localhost:3000")
        .unwrap();
        Self {}
    }
}
