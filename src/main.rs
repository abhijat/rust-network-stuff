extern crate rand;

use rand::{Rng, thread_rng};
use std::io::{Error, Read, Write};
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    println!("incoming connection from {}", stream.peer_addr()?);
    let mut buf = [0; 512];
    loop {
        let bytes_read = stream.read(&mut buf)?;

        if bytes_read == 0 {
            println!("connection closed for {}", stream.peer_addr()?);
            return Ok(());
        } else {
            let data = std::str::from_utf8(&buf[..bytes_read])
                .unwrap()
                .trim();
            println!("client says [{}]", data);
        }

        let sleep = Duration::from_secs(*thread_rng().choose(&[0, 1, 2, 3, 4, 5]).unwrap());
        println!("sleeping for {:?} before replying", sleep);
        std::thread::sleep(sleep);

        stream.write(&buf[..bytes_read])?;
    }
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8888")
        .expect("could not bind");

    for stream in listener.incoming() {
        match stream {
            Err(e) => {
                eprintln!("failed: {}", e)
            }
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream).unwrap_or_else(|error| {
                        eprintln!("error in handle_client: {:?}", error);
                    })
                });
            }
        }
    }
}
