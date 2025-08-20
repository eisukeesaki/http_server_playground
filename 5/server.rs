use std::io::Write;

fn main() -> Result<(), std::io::Error> {
    let ipaddr = "127.0.0.1";
    let port = "4242";
    let socket = format!("{}:{}", ipaddr, port);
    let listener = std::net::TcpListener::bind(socket)?;
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("connected to client");
                let status_line = "HTTP/1.1 200 OK\r\n\r\n";
                let response = status_line.as_bytes();
                stream.write_all(response)?;
                stream.flush()?;
                println!("sent to client: {}", status_line);
            }
            Err(e) => {
                println!("failed to connect to client. {}", e);
                return Err(e);
            } 
        }
    }
    Ok(())
}

