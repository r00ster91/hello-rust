use hello::ThreadPool;
use std::{
    fs,
    io::{Read, Write},
    net, thread,
    time::Duration,
};
mod telephone;

fn main() {
    let port = telephone::r#type("Rust");
    dbg!(port);
    let addr = format!("localhost:{}", port);
    let listener = net::TcpListener::bind(addr).unwrap();

    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        pool.execute(|| {
            handle_connection(stream.unwrap());
        });
    }

    println!("shutting down!");
}

fn handle_connection(mut stream: net::TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    //dbg!(String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename, title) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html", "Hello!")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        (
            "HTTP/1.1 200 OK\r\n\r\n",
            "hello.html",
            "Hello! [Sleep™ Edition]",
        )
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html", "Whoops!")
    };

    let mut contents = fs::read_to_string(filename).unwrap();

    let head = contents.find("<head>").unwrap() + 6;
    let title = format!("<title>{}</title>", &title);
    contents.insert_str(head, &title);

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();

    // wait and prevent the program from continuing until all the bytes are written to the connection
    stream.flush().unwrap();
}
