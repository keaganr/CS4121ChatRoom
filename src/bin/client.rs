use std::io::TcpStream;
use std::io;



fn main() {

	// create stream and input reader
	let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();
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
	let mut message = String::from_str("1");
	message.push_str(String::from_char(1, (user.len() as u8) as char).as_slice());
	message.push_str(user.as_slice());
	message.push_str(String::from_char(1, (pass.len() as u8) as char).as_slice());
	message.push_str(pass.as_slice());
	
	write_message(message, stream.clone());

	// read op code for login
    stream.read(&mut buf);
    let mut op = buf[0] as char;

    println!("op code: {}", op);

    // if op code is 1 login is successful and the client is moved on to the message loop
    if op == '1' {
    	println!("Welcome to the chatroom!\nThere is a 255 character limit on messages and you can type enter 'exit' to exit the application.");

    	let mut input = "".to_string();

    	// user input loop
    	loop { 

    		// get user input
    		input = reader.read_line().ok().expect("Failed to read line.");
    		input.pop();

    		// if user input is "exit" then exit both the client and server process
    		if (input.as_slice() == "exit") {
    			println!("leaving");
    			message = String::from_str("0");
    			write_message(message, stream.clone());
    			break;
    		}

    		// else send a message to the server
    		else {
    			message = String::from_str("2");
    			message.push_str(String::from_char(1, (input.len() as u8) as char).as_slice());
    			message.push_str(input.as_slice());
    			write_message(message, stream.clone());
    		}
    	}

    	drop(stream);
    }
    // if op code is 1 login is failed and the client exits
    else if op == '2' {
    	drop(stream);
    	println!("exit: login failed");
    }
    else { println!("invalid op code"); drop(stream); }
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