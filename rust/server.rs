// This code creates a basic server that listens on port 3000 of the localhost. When a request is made to the server, it responds with a "Hello, World!" message. You can run this code by saving it to a file (e.g. server.rs) and then compiling it with the command  rustc server.rs . After that, you can run the server with the command  ./server .

use std::io::prelude::*;
use std::net::TcpListener;
fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("localhost:3000")?;
    for stream in listener.incoming() {
        let mut stream = stream?;
        let response = "HTTP/1.1 200 OK\r\n\r\nHello, World!";
        stream.write(response.as_bytes())?;
    }
    Ok(())
}