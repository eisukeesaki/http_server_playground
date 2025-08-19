fn main() -> Result<(), std::io::Error> {
    let ipaddr = "127.0.0.1";
    let port = "4242";
    let socket = format!("{}:{}", ipaddr, port);
    let listener = std::net::TcpListener::bind(socket)?;
    let local_addr = std::net::TcpListener::local_addr(&listener)?;
    println!("TCP Listener bound to {local_addr}");

    let (mut stream, _) = listener.accept()?;
    let status_line = "HTTP/1.1 200 OK\r\n\r\n";
    let response = status_line.as_bytes();
    std::io::Write::write_all(&mut stream, response)?;

    Ok(())
}

