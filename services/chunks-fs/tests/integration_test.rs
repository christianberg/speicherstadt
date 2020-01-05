extern crate chunks_fs;
use duct::cmd;
use reqwest;

struct TestServer {
    handle: duct::Handle,
}

impl TestServer {
    fn new() -> Self {
        let handle = {
            let server = cmd!("../../target/debug/chunks-fs")
                .unchecked()
                .stderr_capture()
                .start()
                .expect("Server failed to start.");
            // The server process needs to run long enough to run into
            // potential errors (e.g. port already in use).
            // TODO: eliminate this race condition.
            std::thread::sleep(std::time::Duration::from_millis(100));
            match server.try_wait() {
                // Ok(None) means the server is still running, continue the test
                Ok(None) => (),
                Ok(Some(output)) => {
                    panic!(
                        "Server exited with {}.\nServer log:\n==============\n{}==============\n",
                        output.status,
                        std::str::from_utf8(&output.stderr).unwrap()
                    );
                }
                Err(_) => panic!("An error occurred during server start."),
            };
            server
        };
        Self { handle }
    }

    fn stop(self) {
        self.handle.kill().unwrap();
    }
}

#[test]
fn hello_world() {
    let server = TestServer::new();
    let text = reqwest::get("http://localhost:3000/")
        .unwrap()
        .text()
        .unwrap();
    assert_eq!(text, "Hello world!");
    server.stop();
}
