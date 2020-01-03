extern crate chunks_fs;
use duct::cmd;
use reqwest;

#[test]
fn hello_world() {
    let server = cmd!("../../target/debug/chunks-fs").start().unwrap();
    let text = reqwest::get("http://localhost:3000/")
        .unwrap()
        .text()
        .unwrap();
    assert_eq!(text, "Hello world!");
    server.kill().unwrap();
}
