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

    let listener = std::net::TcpListener::bind(socket)?;
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => println!("handling connection"),
            Err(e) => {
                println!("{:?}", e);
                std::process::exit(1);
            }
        }
    }
}

