use std::io::TcpStream;
use std::io;

fn main() {

	// create stream and input reader
	let mut stream = TcpStream::connect("127.0.0.1", 8080).unwrap();
	let mut reader = io::stdin();

	// get username and password on start, the calls to pop remove the return character
	println!("Enter username: ");
	let mut user = reader.read_line().ok().expect("Failed to read line.");
	user.pop();
	println!("Enter password: ");
	let mut pass = reader.read_line().ok().expect("Failed to read line.");
	pass.pop();

	// create message string to pass to the server as bytes
	let message = String::from_str("1")
					.append(String::from_char(1, (user.len() as u8) as char).as_slice())
					.append(user.as_slice())
					.append(String::from_char(1, (pass.len() as u8) as char).as_slice())
					.append(pass.as_slice());
	let bytes = message.into_bytes();
	let mut buf = [1u8];

	// pass message to server
	for n in range(0u, bytes.len()) {
		buf[0] = bytes[n];
		stream.write(&buf);
	}

	// assumed entry there's a lot left to do here
	println!("Welcome to the chatroom");

	// drop stream and exit
	drop(stream);
	println!("exit");
}