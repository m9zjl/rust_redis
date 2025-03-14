mod resp;
mod resp_result;

use tokio::net::{TcpListener, TcpStream};

use std::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379").await?;
    loop {
        match listener.accept().await {
            Ok((stream, _addr)) => {
                tokio::spawn(process_socket(stream));
            }
            Err(err) => {
                eprintln!("{}", err);
                continue;
            }
        }
    }
}

async fn process_socket(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    loop {
        match stream.read(&mut buffer).await {
            Ok(n) if n != 0 => {
                let response = "PONG\r\n";
                if let Err(err) = stream.write_all(response.as_bytes()).await {
                    eprintln!("{}", err);
                }
            }
            Ok(_) => {
                println!("Connection closed.");
                return;
            }
            Err(err) => {
                eprintln!("{}", err);
                return;
            }
        }
    }
}

// fn handle_connection(stream: &mut TcpStream) {
// }
