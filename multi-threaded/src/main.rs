use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

use webserver::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4).expect("unable to create thread pool");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let reader = BufReader::new(&mut stream);
    let request_line = reader.lines().next().unwrap().unwrap();
    println!("received request: {request_line}");

    let (status_line, filename) = match request_line.as_str() {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "../hello.html"),
        "GET /sleep HTTP/1.1" => {
            println!("sleeping...");
            std::thread::sleep(std::time::Duration::from_secs(5));
            println!("done sleeping");
            ("HTTP/1.1 200 OK", "../hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "../404.html"),
    };
    let contents = std::fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    println!("sending response: {status_line}");
    stream.write_all(response.as_bytes()).unwrap();
}
