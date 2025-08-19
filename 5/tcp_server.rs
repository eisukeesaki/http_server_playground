fn main() -> Result<(), std::io::Error> {
    let ipaddr = "127.0.0.1";
    let port = "4242";
    let socket = format!("{}:{}", ipaddr, port);
    let listener = std::net::TcpListener::bind(socket)?;
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => println!("connected to client"),
            Err(e) => println!("failed to connect to client. {}", e)
        }
    }
    Ok(())
}


