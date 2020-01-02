extern crate iron;

use iron::prelude::*;
use iron::status;
use iron::Handler;

struct ChunkHandler {}

impl Handler for ChunkHandler {
    fn handle(&self, _req: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Hello world!")))
    }
}

pub struct Server {
    pub port: u16,
}

impl Server {
    pub fn start(&self) {
        Iron::new(ChunkHandler {})
            .http(("localhost", self.port))
            .expect("Unable to start server");
    }
}
