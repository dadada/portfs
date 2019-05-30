use std::net::{TcpListener, TcpStream, SocketAddr};
use std::io::Result;
use std::fs::File;
use std::io::Write;
use std::io;
use std::path::Path;

fn handle_client(mut stream: TcpStream, addr: SocketAddr) {
    println!("Received connection from {:?}", addr.port());
    let mut filename = addr.port().to_string();
    if Path::new(&filename).exists() {
        let mut buffer = File::open(&filename).unwrap();
        match io::copy(&mut buffer, &mut stream) {
            Ok(_written) => (),
            Err(e) => println!("Failed to file {:?} to TCP stream: {:?}", filename, e),
        }
    } else {
        let mut buffer = File::create(&filename).unwrap();
        match io::copy(&mut stream, &mut buffer) {
            Ok(_written) => {
                filename.push('\n');
                match stream.write_all(filename.as_bytes()) {
                    Err(e) => println!("Error writing to socket {:?}", e),
                    _ => (),
                }
            }
            Err(e) => println!("Failed to write TCP stream to file {:?} failed with {:?}", filename, e),
        }
    }
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
