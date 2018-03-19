use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:8888")
        .expect("Could not bind socket");

    loop {
        let mut buf = [0u8; 1500];
        let sock = socket.try_clone()
            .expect("failed to clone socket");

        match socket.recv_from(&mut buf) {
            Ok((_, source)) => {
                std::thread::spawn(move || {
                    println!("handling connection from: {}", source);
                    sock.send_to(&buf, &source)
                        .expect("failed to send a response");
                });
            },
            Err(e) => {
                eprintln!("could not receive a datagram: {}", e);
            }
        }
    }
}