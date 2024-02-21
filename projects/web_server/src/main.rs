use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread::{self, Builder, Thread},
    time::Duration,
};

struct ThreadPool {
    threads: Vec<Builder>,
}

impl ThreadPool {
    fn new(num_threads: i32) -> ThreadPool {
        let mut pool = ThreadPool { threads: vec![] };
        for _ in 0..num_threads {
            let thread = thread::Builder::new();
            pool.threads.push(thread);
        }
        pool
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let mut pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        println!("Amount of available threads: {}", pool.threads.len());
        let stream = stream.unwrap();

        let builder = pool.threads.pop().unwrap();

        let _handler = builder
            .spawn(|| {
                handle_connection(stream);
            })
            .unwrap();

        let thread = thread::Builder::new();
        pool.threads.push(thread);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let request_line = &http_request[0][..];
    println!("{request_line}");
    let (status_line, filename) = match request_line {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "index.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
