extern crate iron;

use iron::prelude::*;
use iron::status;

pub struct Service {}

impl Service {
    pub fn new(root_logger: slog::Logger) -> Self {
        let logger = root_logger.new(o!());
        Iron::new(move |_: &mut Request| -> IronResult<Response> {
            debug!(logger, "Serving hello request");
            Ok(Response::with((status::Ok, "Hello world!")))
        })
        .http("localhost:3000")
        .unwrap();
        Self {}
    }
}
