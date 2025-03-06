use std::io;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut num = 12;
    num = 13;
    println!("Listening on {num}");

    return Result::Ok(());

    // // 监听地址: 127.0.0.1:7878
    // let listener = TcpListener::bind("127.0.0.1:8080")?;
    // loop {
    //     match listener.accept() {
    //         Ok((stream, _socket_addr)) => {
    //             tokio::spawn(handle_connection(stream));
    //         }
    //         Err(e) => {
    //             eprintln!("accept error: {}", e);
    //             continue;
    //         }
    //     }
    // }
}

async fn handle_connection(mut stream: TcpStream) {
    let mut buff = [0; 512];

    loop {
        match stream.read(&mut buff) {
            Ok(size) if size != 0 => {
                let response = "+PONG\r\n";
                if let Err(e) = stream.write_all(response.as_bytes()) {
                    eprintln!("Failed to send response: {}", e);
                };
            }
            Ok(_) => {
                println!("Response: {}", String::from_utf8_lossy(&buff[..]));
            }
            Err(_e) => {
                eprintln!("Failed to read from stream");
                return;
            }
        }
    }
}
