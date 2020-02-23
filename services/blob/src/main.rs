extern crate blob;
#[macro_use]
extern crate log;

use blob::start_server;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Config {
    #[structopt(long, env)]
    port: u16,
}

fn main() {
    pretty_env_logger::init();
    let config = Config::from_args();
    info!("Starting server on port {}", config.port);
    start_server(config.port).expect("Server start failed.");
}
