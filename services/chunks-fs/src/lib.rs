extern crate iron;

use iron::prelude::*;
use iron::status;

pub fn start_server(port: u16) {
    Iron::new(hello_handler)
        .http(("localhost", port))
        .expect("Unable to start server");
}

fn hello_handler(_req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Hello world!")))
}
