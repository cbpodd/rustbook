//! # Web Server
//!
//! A simple web server.

use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use webserver::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878")
        .expect("Not handling failures for now");
    let pool = ThreadPool::new(4.try_into().expect("4 is greater than 0"));

    for stream in listener.incoming() {
        let stream = stream.expect("Not handling failures for now");
        pool.execute(|| handle_connection(stream));
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader
        .lines()
        .next()
        .expect("Not handling errors")
        .expect("Not handling erors");

    let (status_line, filename) = match request_line.as_str() {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "html/hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(30));
            ("HTTP/1.1 200 OK", "html/hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "html/404.html"),
    };

    let contents =
        fs::read_to_string(filename).expect("Not handling errors for now");
    let response = format_http_response(status_line, &contents);

    stream
        .write_all(response.as_bytes())
        .expect("Not handling errors");
}

fn format_http_response(status_line: &str, contents: &str) -> String {
    let len = contents.len();
    format!("{status_line}\r\nContent-Length: {len}\r\n\r\n{contents}")
}
