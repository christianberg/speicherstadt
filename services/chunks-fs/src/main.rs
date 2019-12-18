extern crate iron;
#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;

use iron::prelude::*;
use iron::status;
use slog::Drain;

fn root_logger() -> slog::Logger {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    slog::Logger::root(drain, o!("service" => "chunks-fs"))
}

struct Service {}

impl Service {
    fn new(root_logger: slog::Logger) -> Self {
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

fn main() {
    let logger = root_logger();
    info!(logger, "Service starting");
    let _service = Service::new(logger);
}
