use std::io;
use std::net::TcpStream;
use std::io::Write;

fn handle_client(mut stream: TcpStream) -> Result<(), io::Error> {
    println!("connected to client");
    let status_line = "HTTP/1.1 200 OK\r\n\r\n";
    let response = status_line.as_bytes();
    stream.write_all(response)?;
    println!("sent to client: {}", status_line);
    Ok(())
}

fn main() -> Result<(), io::Error> {
    let ipaddr = "127.0.0.1";
    let port = "4242";
    let socket = format!("{}:{}", ipaddr, port);
    let listener = std::net::TcpListener::bind(socket)?;
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_client(stream)?,
            Err(e) => {
                println!("failed to connect to client. {}", e);
                return Err(e);
            } 
        }
    }
    Ok(())
}

