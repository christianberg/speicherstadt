extern crate iron;

#[macro_use]
extern crate slog;

use iron::prelude::*;
use iron::status;
use router::Router;
use slog::Logger;

struct HelloHandler {
    logger: Logger,
}

impl HelloHandler {
    fn new(parent_logger: &Logger) -> Self {
        Self {
            logger: parent_logger.new(o!("route"=>"hello")),
        }
    }
}

impl iron::Handler for HelloHandler {
    fn handle(&self, _req: &mut Request) -> IronResult<Response> {
        info!(self.logger, "Handling hello request");
        Ok(Response::with((status::Ok, "Hello world!")))
    }
}

pub fn start_server(port: u16, base_dir: std::path::PathBuf, parent_logger: &Logger) {
    let put_chunk_logger = parent_logger.new(o!("route" => "put_chunk"));
    let mut router = Router::new();
    router.get("/", HelloHandler::new(parent_logger), "hello");
    router.put(
        "/chunks/sha256/:hash",
        move |req: &mut Request| -> IronResult<Response> {
            info!(put_chunk_logger, "Handling put_chunk request");
            match store_chunk(
                &base_dir,
                req.extensions
                    .get::<Router>()
                    .unwrap()
                    .find("hash")
                    .unwrap(),
                &mut req.body,
                &put_chunk_logger,
            ) {
                Ok(_) => Ok(Response::with(status::Created)),
                Err(e) => {
                    error!(put_chunk_logger, "{}", e);
                    Err(IronError::new(e, status::InternalServerError))
                }
            }
        },
        "put_chunk",
    );
    Iron::new(router)
        .http(("localhost", port))
        .expect("Unable to start server");
}

fn store_chunk(
    base_dir: &std::path::PathBuf,
    hash: &str,
    content: &mut dyn std::io::Read,
    logger: &Logger,
) -> std::io::Result<()> {
    debug!(logger, "Storing chunk {}", hash);
    let mut path = base_dir.clone();
    path.push("sha256");
    path.push(hash);
    debug!(logger, "Storage location: {:?}", path);
    let file = &mut std::fs::File::create(path)?;
    std::io::copy(content, file).map(|size| {
        debug!(logger, "{} bytes written", size);
    })
}
