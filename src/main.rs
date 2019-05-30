use std::net::{TcpListener, TcpStream, SocketAddr};
use std::io::Result;

fn handle_client(stream: TcpStream, addr: SocketAddr) {
    println!("Hello, world!");
}

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    // accept connections and process them serially
    loop {
        match listener.accept() {
            Ok((socket, addr)) => handle_client(socket, addr),
            Err(e) => println!("could not accept client: {:?}", e),
        }

    }
}
