extern crate blob;

use blob::start_server;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Config {
    #[structopt(long, env)]
    port: u16,
}

fn main() {
    let config = Config::from_args();
    start_server(config.port).expect("Server start failed.");
}
