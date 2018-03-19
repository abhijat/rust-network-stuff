use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:8000")
        .expect("could not bind client socket");

    socket.connect("127.0.0.1:8888")
        .expect("could not connect to server");

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)
            .expect("failed to read from stdin");

        if input.to_lowercase().trim() == "quit" {
            println!("good bye!");
            return;
        }

        socket.send(input.as_bytes())
            .expect("failed to send data to socket");

        let mut buffer = [0u8; 1500];
        socket.recv_from(&mut buffer)
            .expect("failed to read from socket");

        let response = std::str::from_utf8(&buffer)
            .expect("failed to convert response to string")
            .trim()
            .replace('\n', "");
        println!("server responds [{}]", response);
    }
}