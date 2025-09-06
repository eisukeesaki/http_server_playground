use std::io::{Read, Write};

// we want to return an Option that indicates either a succession or a failure of 
// this functino to main.
// what form could the return type take?
fn handle_connection(mut stream: std::net::TcpStream) -> Result<(), std::io::Error> {
    let mut buff = vec![0; 512];
    match stream.read(&mut buff) {
        Ok(_) => {
            let read = std::string::String::from_utf8_lossy(&buff);
            let request = read.trim();
            let request_line = match request.lines().next() {
                Some(request_line) => request_line,
                None => ""
            };
            let request_target = match request_line.split_whitespace().nth(1) {
                Some(request_target) => request_target,
                None => ""
            };
            let response;
            if request_target == "/" {
                response = "HTTP/1.1 200 OK\r\n\r\n";
            }
            else {
                response = "HTTP/1.1 404 Not Found\r\n\r\n";
            }

            match stream.write_all(&response.as_bytes()) {
                Ok(_) => {
                    Ok(())
                }
                Err(e) => {
                    Err(e)
                }
            }
        }
        Err(e) => {
            eprintln!("{}", e);
            Err(e)
        }
    }
}

fn main() {
    let ipaddr = "127.0.0.1";
    let port = "4242";
    let socket = format!("{}:{}", ipaddr, port);
    let listener = match std::net::TcpListener::bind(socket) {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };
    match listener.accept() {
        Ok((stream, _)) => match handle_connection(stream) {
            Ok(_) => return,
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        },
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}

