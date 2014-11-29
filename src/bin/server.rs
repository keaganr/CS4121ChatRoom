use std::io::{TcpListener, TcpStream, Acceptor, Listener};
use std::str;

fn main() {

	// create listener and bind it
	let listener = TcpListener::bind("127.0.0.1", 8080);

	// start listen
	let mut acceptor = listener.listen();

	// accept connections and spawn tasks for each
	for stream in acceptor.incoming() {
	    match stream {
	        Err(e) => { /* connection failed */ }
	        Ok(stream) => spawn(proc() {
	        	println!("got client");
	            handle_client(stream);
	        })
	    }
	}

	// close the socket server
	drop(acceptor);
}

// handle the spawned client task
fn handle_client(mut stream: TcpStream) {

    let mut buf = [1u8];

    // read op code and print it
    stream.read(buf);
    let op = String::from_byte(buf[0]);

    if op.as_slice() == "1" { login(stream); }
    else if op.as_slice() == "2" { send_all(); }
    else if op.as_slice() == "3" { send_hist(); }
    else if op.as_slice() == "4" { announce(); }
    //println!("{}", op);
}

// SOP1: login
fn login(mut stream: TcpStream) {
	let mut buf = [1u8];
	let mut user = "".to_string();
	let mut pass = "".to_string();

	// get the username length and read it into the user variable
	stream.read(buf);
	for n in range(0u, buf[0] as uint) {
		stream.read(buf);
		user = user.append(String::from_byte(buf[0]).as_slice());
	}

	// get the password length and read it into the pass variable
	stream.read(buf);
	for n in range(0u, buf[0] as uint) {
		stream.read(buf);
		pass = pass.append(String::from_byte(buf[0]).as_slice());
	}

	println!("start login, username: {} password: {}", user, pass);
}

// SOP2: send_all
fn send_all() {
	println!("start send all");
}

// SOP3: send_hist
fn send_hist() {
	println!("start send history");
}

// SOP4: announce
fn announce() {
	println!("start announce");
}