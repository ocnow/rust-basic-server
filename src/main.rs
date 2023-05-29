use hello::Threadpool;
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = Threadpool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("connection enstabilished");
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn r#handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    // map(|result| result.unwrap())
    // .take_while(|line| !line.is_empty())
    // .collect();

    // println!("Requests : {:#?}",http_requests);

    let (status_line, fileName) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "src/hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "src/hello.html")
        }
        _ => ("HTTP/1.1 404 OK", "src/404.html"),
    };

    let contents = fs::read_to_string(fileName).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
