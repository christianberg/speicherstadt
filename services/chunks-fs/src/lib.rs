extern crate iron;

#[macro_use]
extern crate slog;

use iron::prelude::*;
use iron::status;
use router::Router;
use slog::Logger;

pub fn start_server(port: u16, parent_logger: &Logger) {
    let hello_logger = parent_logger.new(o!("route" => "hello"));
    let put_chunk_logger = parent_logger.new(o!("route" => "put_chunk"));
    let mut router = Router::new();
    router.get(
        "/",
        move |_req: &mut Request| -> IronResult<Response> {
            info!(hello_logger, "Handling hello request");
            Ok(Response::with((status::Ok, "Hello world!")))
        },
        "hello",
    );
    router.put(
        "/chunks/sha256/:hash",
        move |_req: &mut Request| -> IronResult<Response> {
            info!(put_chunk_logger, "Handling put_chunk request");
            Ok(Response::with(status::Created))
        },
        "put_chunk",
    );
    Iron::new(router)
        .http(("localhost", port))
        .expect("Unable to start server");
}
