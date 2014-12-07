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
    let mut user = "".to_string();


    // read op code
    stream.read(buf);
    let mut op = String::from_byte(buf[0]);

    // loop for client operations
    while op.as_slice() != "0" {
	    if op.as_slice() == "1" {
	    	user = login(stream.clone());
	    	if user.as_slice() != "" { 
	    		write_message(String::from_str("1"), stream.clone());
	    	}
	    	else {
	    		write_message(String::from_str("2"), stream.clone());
	    		break;
	    	}
	    }
	    else if op.as_slice() == "2" { send_all(user.clone(), stream.clone()); }
	    else if op.as_slice() == "3" { send_hist(); }
	    else if op.as_slice() == "4" { announce(); }
	    else { fail!("invalid op code"); }

	    // read new op code
    	stream.read(buf);
    	op = String::from_byte(buf[0]);
	}

    drop(stream);
    println!("exiting client task");
}

// SOP1: login
// reads in user credentials and returns true if they can be accepted
// and false if they cannot
fn login(mut stream: TcpStream) -> String {

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

	// test if user and pass are correct, if they are the user is returned,
	// if they aren't an empty string is returned
	if user.as_slice() == "user" && pass.as_slice() == "pass" { 
		println!("acceptable credentials");
		return user;

	}
	else {println!("failure!!!"); return "".to_string();}
}

// SOP2: send_all
fn send_all(user: String, mut stream: TcpStream) {
	let mut buf = [1u8];
	let mut text = "".to_string();

	stream.read(buf);
	for n in range(0u, buf[0] as uint) {
		stream.read(buf);
		text = text.append(String::from_byte(buf[0]).as_slice());
	}

	println!("got text: {}", text);

	//TODO: implement sending received message to all clients
	
}

// SOP3: send_hist
fn send_hist() {
	println!("start send history");
}

// SOP4: announce
fn announce() {
	println!("start announce");
}

// function to pass a string message into a stream byte by byte
fn write_message(message : String, mut stream: TcpStream) {
	let bytes = message.into_bytes();
	let mut buf = [1u8];

	// pass message to server
	for n in range(0u, bytes.len()) {
		buf[0] = bytes[n];
		stream.write(&buf);
	}
}