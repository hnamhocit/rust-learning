fn main() {
    // Simple HTTP Server
    use std::io::{Read, Write};
    use std::net::{TcpListener, TcpStream};
    use std::thread;

    fn handle_client(mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();

        let response = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n<html><body><h1>Hello from Rust Server!</h1></body></html>\r\n";
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }

    fn start_server() {
        let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
        println!("Server started at http://127.0.0.1:8080");

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    thread::spawn(|| {
                        handle_client(stream);
                    });
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
    }

    start_server();
}
