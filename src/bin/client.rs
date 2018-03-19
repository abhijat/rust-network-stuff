use std::io::{self, BufRead, BufReader, Write};
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;

fn main() {
    let remote: SocketAddr = "127.0.0.1:8888".parse().unwrap();
    let timeout = Duration::from_secs(1);

    let mut stream = TcpStream::connect_timeout(&remote, timeout)
        .expect("could not connect to server");

    let _ = stream.set_read_timeout(Some(Duration::from_secs(3)));

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("failed to read from stdin");

        if input.trim() == "quit" {
            println!("good bye!");
            return;
        }

        stream.write(input.as_bytes())
            .expect("failed to write to server");

        let mut buffer: Vec<u8> = Vec::new();
        BufReader::new(&stream).read_until(b'\n', &mut buffer)
            .expect("could not read into buffer");

        print!("{}", std::str::from_utf8(&buffer)
            .expect("could not write buffer as string"));
    }
}