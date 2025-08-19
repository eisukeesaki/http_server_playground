use std::net::TcpListener;
use std::io::Write;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        println!("Terminating due to insufficent number of arguments.");
        println!("usage: ./app <ipaddr> <port>");
        std::process::exit(1);
    }

    let ipaddr = &args[1];
    let port = &args[2];
    let socket = format!("{}:{}", ipaddr, port);
    let listener = match TcpListener::bind(&socket) {
        Ok(l) => {
            println!("Server bound to {}", socket);
            l
        },
        Err(e) => {
            eprintln!("Could not bind to {}. {}", socket, e);
            std::process::exit(1);
        }
    };

    match listener.accept() {
        Ok((mut stream, socket)) => {
            println!("Accepted connection request from {}", socket);
            let status = "HTTP/1.1 200 OK\r\n\r\n";
            match stream.write(status.as_bytes()) {
                Ok(size_written) => println!("{} bytes written to {}",
                    size_written, socket),
                Err(e) => eprintln!("Failed to write to {} {}", socket, e)
            };
        },
        Err(e) => {
            eprintln!("Failed to accept connection from {} {}", socket, e);
            std::process::exit(1);
        }
    };
}

