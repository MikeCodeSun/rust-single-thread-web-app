use std::{net::{TcpListener, TcpStream}, io::{BufReader, BufRead, Write}, fs};

fn main() {

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_stream(stream);
    }
}

fn handle_stream(mut stream: TcpStream) {
    let buf_stream = BufReader::new(&mut stream);
    let request_line = buf_stream.lines().next().unwrap().unwrap();

    let (status, file_path) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        "GET /about HTTP/1.1" => ("HTTP/1.1 200 OK", "about.html"),
        _ => ("HTTP/1.1 200 OK", "404.html"),
    };

    let contents = fs::read_to_string(file_path).unwrap();
    let length = contents.len();
    let res = format!("{status}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(&res.as_bytes()).unwrap();
}
