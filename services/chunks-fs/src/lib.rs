extern crate iron;

use iron::prelude::*;
use iron::status;
use iron::Handler;

pub struct ChunkHandler {}

impl Handler for ChunkHandler {
    fn handle(&self, _req: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Hello world!")))
    }
}
