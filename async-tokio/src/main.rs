// use std::{
//     io::{BufRead, BufReader, Write},
//     net::{TcpListener, TcpStream},
// };

use tokio::{net::{TcpListener, TcpStream}, io::{BufReader, AsyncBufReadExt, AsyncWriteExt}};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();
    loop {
        let (stream, _addr) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            println!("tokio task spawned");
            handle_connection(stream).await;
            println!("tokio task finished");
        });
    }
}

async fn handle_connection(mut stream: TcpStream) {
    // get request asynchronously
    let reader = BufReader::new(&mut stream);
    let request_line = reader.lines().next_line().await.unwrap().unwrap();
    println!("received request: {request_line}");

    // get response asynchronously
    let (status_line, filename) = match request_line.as_str() {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "../hello.html"),
        "GET /sleep HTTP/1.1" => {
            println!("sleeping...");
            // non blocking sleep
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            println!("done sleeping");
            ("HTTP/1.1 200 OK", "../hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "../404.html"),
    };
    let contents = std::fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    // send response asynchronously
    println!("sending response: {status_line}");
    stream.write_all(response.as_bytes()).await.unwrap();
}
