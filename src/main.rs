extern crate tokio;

use std::path::Path;
use tokio::fs::File;
use tokio::io::copy;
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;
use tokio::prelude::{AsyncRead, Future};

fn main() {
    let addr = "127.0.0.1:8080".parse().unwrap();
    let listener = TcpListener::bind(&addr).expect("unable to bind TCP listener");

    let server = listener
        .incoming()
        .map_err(|e| eprintln!("accept failed = {:?}", e))
        .for_each(|sock| {
            let addr = sock.peer_addr().unwrap();
            println!("Received connection from {:?}", addr.port());

            let filename = addr.port().to_string();
            let path = Path::new(&filename);

            let (reader, writer) = sock.split();

            if path.exists() {
                let task = File::open(filename)
                    .and_then(|file| copy(file, writer))
                    .map(|res| println!("{:?}", res))
                    .map_err(|err| eprintln!("IO error: {:?}", err));
                tokio::spawn(task)
            } else {
                let task = File::create(filename)
                    .and_then(|file| copy(reader, file))
                    .map(|res| println!("{:?}", res))
                    .map_err(|err| eprintln!("IO error: {:?}", err));
                tokio::spawn(task)
            }

        });

    // Start the Tokio runtime
    tokio::run(server);
}
