use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    // 监听地址: 127.0.0.1:7878
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream);
            }
            Err(_e) => {}
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buff = [0; 512];

    loop {
        match stream.read(&mut buff) {
            Ok(size) if size != 0 => {
                let response = "+PONG\r\n";
                stream.write(response.as_bytes()).unwrap();
                stream.flush().unwrap();
            }
            Ok(_) => {}
            Err(_e) => {}
        }
    }

    stream.read(&mut buff).unwrap();

    print!("Received: {}", String::from_utf8_lossy(&buff[..]));
    let response = "+PONG\r\n";
    stream.write(response.as_bytes()).unwrap();
}
