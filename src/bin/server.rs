use std::io::{TcpListener, TcpStream, Acceptor, Listener};
use std::str;

fn main() {

	// create listener and bind it
	let listener = TcpListener::bind("127.0.0.1", 8080);

	// start listen
	let mut acceptor = listener.listen();

	fn handle_client(mut stream: TcpStream) {

	    let mut buf = [1u8];

	    // read op code and print it
	    stream.read(buf);
	    let op = String::from_byte(buf[0]);
	    println!("{}", op);
	}
	// accept connections and spawn tasks for each
	for stream in acceptor.incoming() {
	    match stream {
	        Err(e) => { /* connection failed */ }
	        Ok(stream) => spawn(proc() {
	        	println!("got client");
	            handle_client(stream)
	        })
	    }
	}

	// close the socket server
	drop(acceptor);
}