extern crate tokio;
extern crate tokio_core;

use tokio::io;
use tokio::net::TcpListener;
use tokio::prelude::*;
use tokio_core::reactor::Core;

fn main() {
    let mut core = Core::new().unwrap();

    let addr = "127.0.0.1:8080".parse().unwrap();
    let listener = TcpListener::bind(&addr).unwrap();

    let server = listener.incoming().for_each(|socket| {
        println!("accepted socket; addr={:?}", socket.peer_addr().unwrap());

        let connection = io::write_all(socket, "hello world\n").then(|res| {
            println!("wrote message; success={:?}", res.is_ok());
            Ok(())
        });

        tokio::spawn(connection);

        Ok(())
    });

    core.run(server).unwrap();
}
