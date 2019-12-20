extern crate chunks_fs;

#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;

use chunks_fs::Service;
use iron::Iron;
use slog::Drain;

fn root_logger() -> slog::Logger {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    slog::Logger::root(drain, o!("service" => "chunks-fs"))
}

fn main() {
    let logger = root_logger();
    info!(logger, "Service starting");
    Iron::new(Service {}).http("localhost:3000").unwrap();
}
