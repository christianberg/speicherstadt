extern crate chunks_fs;
use duct::cmd;
use reqwest;
use std::io::Read;

struct TestServer {
    handle: duct::Handle,
    base_dir: tempfile::TempDir,
}

impl TestServer {
    fn new(port: u16) -> Self {
        let base_dir = tempfile::tempdir().unwrap();
        println!("{:?}", base_dir);
        let handle = {
            let server = cmd!("../../target/debug/chunks-fs")
                .env("PORT", format!("{}", port))
                .env("BASE_DIR", base_dir.path().as_os_str())
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
        Self { handle, base_dir }
    }
}

impl Drop for TestServer {
    fn drop(&mut self) {
        self.handle.kill().unwrap();
    }
}

#[test]
fn upload_chunk() {
    let server = TestServer::new(3002);
    let input: Vec<u8> = "hello world".as_bytes().to_vec();
    let url = "http://localhost:3002/chunks/sha256/b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9";
    let client = reqwest::Client::new();
    let put_result = client.put(url).body(input.clone()).send().unwrap();
    assert!(put_result.status().is_success());
    let mut get_result = client.get(url).send().unwrap();
    assert!(get_result.status().is_success());
    let mut output = Vec::new();
    get_result.read_to_end(&mut output).unwrap();
    assert_eq!(input, output);
}
