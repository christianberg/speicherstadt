extern crate chunks_fs;

#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;

use chunks_fs::Server;
use slog::Drain;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Config {
    #[structopt(long, env)]
    port: u16,
}

fn root_logger() -> slog::Logger {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    slog::Logger::root(drain, o!("service" => "chunks-fs"))
}

fn main() {
    let config = Config::from_args();
    let logger = root_logger();
    info!(logger, "Service starting on port {}", config.port);
    (Server { port: config.port }).start();
}
