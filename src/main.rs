use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use std::{fs, thread};

use http_server::ThreadPool;

fn main() {
    // bind to ip address and port
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let thread_pool = ThreadPool::new(10);

    // iterate over the connection attempts (hence this is Result<>)
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread_pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buffer_reader = BufReader::new(&stream);
    // rather than reading the entire request into a vector, we’re calling next to get the first item from the iterator
    let request_line = buffer_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "static/index.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "static/index.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "static/404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
