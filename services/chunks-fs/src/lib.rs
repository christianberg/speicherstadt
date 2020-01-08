extern crate iron;

#[macro_use]
extern crate slog;

use iron::prelude::*;
use iron::status;
use router::Router;
use slog::Logger;
use std::path::PathBuf;

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

    fn handle_put(&self, req: &mut Request) -> IronResult<Response> {
        info!(self.logger, "Handling PUT chunk request");
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

    fn handle_get(&self, req: &mut Request) -> IronResult<Response> {
        info!(self.logger, "Handling GET chunk request");
        match self.storage.retrieve_chunk(
            req.extensions
                .get::<Router>()
                .unwrap()
                .find("hash")
                .unwrap(),
        ) {
            Ok(file) => Ok(Response::with((status::Ok, file))),
            Err(e) => {
                error!(self.logger, "{}", e);
                Err(IronError::new(e, status::InternalServerError))
            }
        }
    }
}

impl iron::Handler for ChunkHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        info!(self.logger, "Handling {} request", req.method);
        match &req.method {
            iron::method::Method::Get => self.handle_get(req),
            iron::method::Method::Put => self.handle_put(req),
            _ => Ok(Response::with(status::MethodNotAllowed)),
        }
    }
}

struct Storage {
    data_dir: PathBuf,
    logger: Logger,
}

impl Storage {
    fn new(base_dir: PathBuf, parent_logger: &Logger) -> Self {
        let mut data_dir = base_dir.clone();
        data_dir.push("data");
        std::fs::create_dir(&data_dir).unwrap();
        data_dir.push("sha256");
        std::fs::create_dir(&data_dir).unwrap();
        Self {
            data_dir,
            logger: parent_logger.new(o!("component"=>"storage")),
        }
    }

    fn path_for_hash(&self, hash: &str) -> PathBuf {
        let mut path = self.data_dir.clone();
        path.push(hash);
        path
    }

    fn store_chunk(&self, hash: &str, content: &mut dyn std::io::Read) -> std::io::Result<()> {
        debug!(self.logger, "Storing chunk {}", hash);
        let path = self.path_for_hash(hash);
        debug!(self.logger, "Storage location: {:?}", path);
        let file = &mut std::fs::File::create(path)?;
        std::io::copy(content, file).map(|size| {
            debug!(self.logger, "{} bytes written", size);
        })
    }

    fn retrieve_chunk(&self, hash: &str) -> std::io::Result<std::fs::File> {
        let path = self.path_for_hash(hash);
        std::fs::File::open(path)
    }
}

pub fn start_server(port: u16, base_dir: PathBuf, logger: &Logger) {
    let mut router = Router::new();
    let storage = Storage::new(base_dir, logger);
    router.get("/", HelloHandler::new(logger), "hello");
    router.any(
        "/chunks/sha256/:hash",
        ChunkHandler::new(storage, logger),
        "chunks",
    );
    Iron::new(router)
        .http(("localhost", port))
        .expect("Unable to start server");
}
