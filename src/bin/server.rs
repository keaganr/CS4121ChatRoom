use std::sync::{Arc, Mutex};
use std::comm::Sender;
use std::io::{TcpListener, TcpStream, Acceptor, Listener};
use std::io;
use std::str;


fn main() {

	// create listener and bind it
	let listener = TcpListener::bind("127.0.0.1:8080");

	// start listen
	let mut acceptor = listener.listen().unwrap();

	let stream_mut = Arc::new(Mutex::new(0u32));
	let (in_tx, in_rx) = channel();
	let close_tx = in_tx.clone();

	let (stream_tx, stream_rx) = channel::<String>();

	let mut stream_vec: Vec<TcpStream> = Vec::new();
	let streams = Arc::new(Mutex::new(stream_vec));




	// spawn a task to handle individual clients
	let mut acceptor_clone = acceptor.clone();
	let mut acceptor_streams = streams.clone();
	spawn(proc() {
		for stream in acceptor_clone.incoming() {
			match stream {
				Err(e) => { 
					println!("server stopping"); 
					break; 
				}
				Ok(stream) => {
					let in_tx = in_tx.clone();
					let mut stream_vec = acceptor_streams.lock();
					(*stream_vec).push(stream.clone());
					spawn(proc() {
						println!("got client");
						handle_client(stream, in_tx);
					});
				}
			}
		}
	});
	
	// spawn task for receiving and sending messages to all clients
	spawn(proc() {
		loop {
			let received = in_rx.recv();
			if received.as_slice() == "exit" { break; }
			let mut message = String::from_str("3");
			message.push_str(received.as_slice());

			let mut rec_vec = streams.lock();
			for s in (*rec_vec).iter() {
				write_message(message.clone(), s.clone());
			}
		}
	});

	// allow user input for exit
	let mut line = io::stdin().read_line().ok().unwrap();
	line = remove_end_newline_char(line);
	while line.to_string() != "exit".to_string() {
		line = io::stdin().read_line().ok().unwrap();
		line = remove_end_newline_char(line);
	}
	
	// drop the Sender end of the channel and close the socket server
	close_tx.send("exit".to_string());
	acceptor.close_accept();
}

fn remove_end_newline_char(message : String) -> String {
	let mut new_message = message.to_string();
	let mut last_char = new_message.pop().unwrap();
	while last_char == 13 as char || last_char == 10 as char {
		last_char = new_message.pop().unwrap();
	}
	new_message.push_str(String::from_char(1, last_char).as_slice());
	return new_message;
}

// handle the spawned client task
fn handle_client(mut stream: TcpStream, mut tx: std::comm::Sender<String>) {

    let mut buf = [1u8];
    let mut user = "".to_string();


    // read op code
    stream.read(&mut buf);
    
    let mut op = buf[0] as char;

    println!("op code: {}", op);

    // loop for client operations
    while op != '0' {
	    if op == '1' {
	    	user = login(stream.clone());
	    	if user.as_slice() != "" { 
	    		write_message(String::from_str("1"), stream.clone());
	    	}
	    	else {
	    		write_message(String::from_str("2"), stream.clone());
	    		break;
	    	}
	    }
	    else if op == '2' { send_all(user.clone(), stream.clone(), tx.clone()); }
	    else if op == '3' { send_hist(); }
	    else if op == '4' { announce(); }
	    else { println!("invalid op code"); break; }

	    // read new op code
    	stream.read(&mut buf);
    	op = buf[0] as char;
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
	stream.read(&mut buf);
	for n in range(0u, buf[0] as uint) {
		stream.read(&mut buf);
		user.push(buf[0] as char);
	}

	// get the password length and read it into the pass variable
	stream.read(&mut buf);
	for n in range(0u, buf[0] as uint) {
		stream.read(&mut buf);
		pass.push(buf[0] as char);
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
fn send_all(user: String, mut stream: TcpStream, mut tx: std::comm::Sender<String>) {
	let mut buf = [1u8];
	let mut text = "".to_string();

	stream.read(&mut buf);
	text.push(buf[0] as char);
	for n in range(0u, buf[0] as uint) {
		stream.read(&mut buf);
		text.push(buf[0] as char);
	}

	tx.send(text);

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