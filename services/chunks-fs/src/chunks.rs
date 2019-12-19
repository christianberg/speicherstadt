extern crate iron;

use iron::prelude::*;
use iron::status;
use iron::Handler;

pub struct Service {}

impl Handler for Service {
    fn handle(&self, _req: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Hello world!")))
    }
}
