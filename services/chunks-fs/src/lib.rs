extern crate iron;

#[macro_use]
extern crate slog;

use iron::prelude::*;
use iron::status;
use router::Router;
use slog::Logger;

pub fn start_server(port: u16, parent_logger: &Logger) {
    let logger = parent_logger.new(o!("foo" => "bar"));
    let mut router = Router::new();
    router.get(
        "/",
        move |_req: &mut Request| -> IronResult<Response> {
            info!(logger, "Handled hello request");
            Ok(Response::with((status::Ok, "Hello world!")))
        },
        "hello",
    );
    Iron::new(router)
        .http(("localhost", port))
        .expect("Unable to start server");
}
