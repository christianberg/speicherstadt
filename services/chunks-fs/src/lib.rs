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

struct ChunkHandler {
    storage: Storage,
    logger: Logger,
}

impl ChunkHandler {
    fn new(storage: Storage, parent_logger: &Logger) -> Self {
        Self {
            storage,
            logger: parent_logger.new(o!("route"=>"chunks")),
        }
    }
}

impl iron::Handler for ChunkHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        info!(self.logger, "Handling put_chunk request");
        match self.storage.store_chunk(
            req.extensions
                .get::<Router>()
                .unwrap()
                .find("hash")
                .unwrap(),
            &mut req.body,
        ) {
            Ok(_) => Ok(Response::with(status::Created)),
            Err(e) => {
                error!(self.logger, "{}", e);
                Err(IronError::new(e, status::InternalServerError))
            }
        }
    }
}

struct Storage {
    data_dir: std::path::PathBuf,
    logger: Logger,
}

impl Storage {
    fn new(base_dir: std::path::PathBuf, parent_logger: &Logger) -> Self {
        let mut data_dir = base_dir.clone();
        data_dir.push("data");
        Self {
            data_dir,
            logger: parent_logger.new(o!("component"=>"storage")),
        }
    }

    fn store_chunk(&self, hash: &str, content: &mut dyn std::io::Read) -> std::io::Result<()> {
        debug!(self.logger, "Storing chunk {}", hash);
        let mut path = self.data_dir.clone();
        path.push("sha256");
        path.push(hash);
        debug!(self.logger, "Storage location: {:?}", path);
        let file = &mut std::fs::File::create(path)?;
        std::io::copy(content, file).map(|size| {
            debug!(self.logger, "{} bytes written", size);
        })
    }
}

pub fn start_server(port: u16, base_dir: std::path::PathBuf, logger: &Logger) {
    let mut router = Router::new();
    let storage = Storage::new(base_dir, logger);
    router.get("/", HelloHandler::new(logger), "hello");
    router.put(
        "/chunks/sha256/:hash",
        ChunkHandler::new(storage, logger),
        "chunks_put",
    );
    Iron::new(router)
        .http(("localhost", port))
        .expect("Unable to start server");
}
