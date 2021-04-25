use std::net::{TcpListener, TcpStream};
use dotenv;

fn handle_client(stream: TcpStream) {
    // ...
}

fn main() -> std::io::Result<()> {
	let url = dotenv::var("SERVER_URL").unwrap();
	println!("{}", url);
    let listener =  TcpListener::bind(url).unwrap();

	// accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}
