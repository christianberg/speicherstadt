extern crate chunks_fs;

#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;

use chunks_fs::start_server;
use slog::Drain;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Config {
    #[structopt(long, env)]
    port: u16,
    #[structopt(long, env)]
    base_dir: std::path::PathBuf,
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
    start_server(config.port, config.base_dir, &logger).expect("Server start failed.");
}
