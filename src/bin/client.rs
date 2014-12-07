use std::io::TcpStream;
use std::io;



fn main() {

	// create stream and input reader
	let mut stream = TcpStream::connect("127.0.0.1", 8080).unwrap();
	let mut reader = io::stdin();
	let mut buf = [1u8];

	// get username and password on start, the calls to pop remove the return character
	println!("Enter username: ");
	let mut user = reader.read_line().ok().expect("Failed to read line.");
	user.pop();
	println!("Enter password: ");
	let mut pass = reader.read_line().ok().expect("Failed to read line.");
	pass.pop();

	// create message string to pass to the server as bytes
	let mut message = String::from_str("1")
					.append(String::from_char(1, (user.len() as u8) as char).as_slice())
					.append(user.as_slice())
					.append(String::from_char(1, (pass.len() as u8) as char).as_slice())
					.append(pass.as_slice());
	
	write_message(message, stream.clone());

	// read op code for login
    stream.read(buf);
    let mut op = String::from_byte(buf[0]);

    // if op code is 1 login is successful and the client is moved on to the message loop
    if op.as_slice() == "1" {
    	println!("Welcome to the chatroom");

    	//TODO: currently just an infinite loop for the message loop
    	loop {}

    	drop(stream);
    }
    // if op code is 1 login is failed and the client exits
    else if op.as_slice() == "2" {
    	drop(stream);
    	println!("exit: login failed");
    }
    else { fail!("invalid op code"); }
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