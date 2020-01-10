extern crate iron;

#[macro_use]
extern crate slog;

use iron::middleware::BeforeMiddleware;
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

fn handle_put(req: &mut Request) -> IronResult<Response> {
    let storage = req.extensions.get::<Storage>().unwrap();
    match storage.store_chunk(
        req.extensions
            .get::<Router>()
            .unwrap()
            .find("hash")
            .unwrap(),
        &mut req.body,
    ) {
        Ok(_) => Ok(Response::with(status::Created)),
        Err(e) => Err(IronError::new(e, status::InternalServerError)),
    }
}

fn handle_get(req: &mut Request) -> IronResult<Response> {
    let storage = req.extensions.get::<Storage>().unwrap();
    match storage.retrieve_chunk(
        req.extensions
            .get::<Router>()
            .unwrap()
            .find("hash")
            .unwrap(),
    ) {
        Ok(file) => Ok(Response::with((status::Ok, file))),
        Err(e) => Err(IronError::new(e, status::InternalServerError)),
    }
}

struct Storage {
    data_dir: PathBuf,
    partial_dir: PathBuf,
    logger: Logger,
}

impl iron::typemap::Key for Storage {
    type Value = ChunkStorer;
}

impl BeforeMiddleware for Storage {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions.insert::<Storage>(self.get_storer());
        Ok(())
    }
}

impl Storage {
    fn new(base_dir: PathBuf, parent_logger: &Logger) -> std::io::Result<Self> {
        let logger = parent_logger.new(o!("component"=>"storage"));
        info!(logger, "Initializing Storage with base dir {:?}", base_dir);
        assert!(base_dir.is_dir());
        let mut data_dir = base_dir.clone();
        data_dir.push("data");
        if !data_dir.exists() {
            info!(logger, "Creating directory {:?}", data_dir);
            std::fs::create_dir(&data_dir)?;
        }
        data_dir.push("sha256");
        if !data_dir.exists() {
            info!(logger, "Creating directory {:?}", data_dir);
            std::fs::create_dir(&data_dir)?;
        }
        let mut partial_dir = base_dir.clone();
        partial_dir.push("partial");
        if !partial_dir.exists() {
            info!(logger, "Creating directory {:?}", partial_dir);
            std::fs::create_dir(&partial_dir)?;
        }
        Ok(Self {
            data_dir,
            partial_dir,
            logger,
        })
    }

    fn get_storer(&self) -> ChunkStorer {
        ChunkStorer {
            data_dir: self.data_dir.clone(),
            logger: self.logger.new(o!()),
        }
    }
}

struct ChunkStorer {
    data_dir: PathBuf,
    logger: Logger,
}

impl ChunkStorer {
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

pub fn start_server(port: u16, base_dir: PathBuf, logger: &Logger) -> Result<(), std::io::Error> {
    let mut router = Router::new();
    let storage = Storage::new(base_dir, logger)?;
    router.get("/", HelloHandler::new(logger), "hello");
    router.get("/chunks/sha256/:hash", handle_get, "chunks_get");
    router.put("/chunks/sha256/:hash", handle_put, "chunks_put");
    let mut chain = Chain::new(router);
    chain.link_before(storage);
    Iron::new(chain)
        .http(("localhost", port))
        .expect("Unable to start server");
    Ok(())
}
