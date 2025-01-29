use indoc::indoc;
use rust_web::ThreadPool;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    // 监听地址: 127.0.0.1:7878
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(6);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| handle_connection(stream));
    }
}

fn handle_connection(mut tcp_stream: TcpStream) {
    let buf_reader = BufReader::new(&mut tcp_stream);
    let _http_request = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect::<Vec<String>>();

    // println!("Request: {:#?}", http_request);
    let status_line = "HTTP/1.1 200 OK";
    let contents = indoc! {"
            <!DOCTYPE html>
            <html lang=\"en\">
            <head>
                <meta charset=\"utf-8\">
                <title>Hello!</title>
            </head>
            <body>
            <h1>Hello!</h1>
            <p>Hi from Rust</p>
            </body>
            </html>
    "};
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    tcp_stream.write_all(response.as_bytes()).unwrap();
}
