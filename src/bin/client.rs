extern crate rustbox;
extern crate core;

use std::sync::{Arc, Semaphore, Mutex};
use std::io::TcpStream;
use std::io;
use core::slice::Items;

use std::char;
use rustbox::{Style,Color};



fn main() {

	let mut stream_fail = false;
	let mut op = '0';
	let mut stream: TcpStream;
	let mut reader = io::stdin();
	let mut buf = [1u8];
	let mut test = 'a';
	let UI_access = Arc::new(Semaphore::new(1));
	let mut message_vec: Vec<String> = Vec::new();
	let messages = Arc::new(Mutex::new(message_vec));

	let mut result = TcpStream::connect("127.0.0.1:8080");
	match result {
		Ok(s) => { 
			stream = s;

			println!("Welcome to the Rust Chatroom Client!\nAvailable commands to enter are:\n'create' - register an account\n'login'  - input credentials and access the chat room");
			let mut option = reader.read_line().ok().expect("Failed to read line.");
			option = remove_end_newline_char(option);

			if (option == "create") {
				// drop(stream);
					// get username and password on start
					print!("Enter username: ");
					let mut user = reader.read_line().ok().expect("Failed to read line.");
					user = remove_end_newline_char(user);

					print!("Enter password: ");
					let mut pass = reader.read_line().ok().expect("Failed to read line.");
					pass = remove_end_newline_char(pass);

					let mut message = String::from_str("4");
					message.push_str(String::from_char(1, (user.len() as u8) as char).as_slice());
					message.push_str(user.as_slice());
					message.push_str(String::from_char(1, (pass.len() as u8) as char).as_slice());
					message.push_str(pass.as_slice());

					write_message(message, stream.clone());
					stream.read(&mut buf);
					op = buf[0] as char;
					if (op == '1') {
						println!("account created - please type your credentials again to log in") ;
					} else {
						println!("account creation failed - please try again later");
						return;
					}
				}


			// get username and password on start
			print!("Enter username: ");
			let mut user = reader.read_line().ok().expect("Failed to read line.");
			user = remove_end_newline_char(user);

			print!("Enter password: ");
			let mut pass = reader.read_line().ok().expect("Failed to read line.");
			pass = remove_end_newline_char(pass);

			// create message string for login to pass to the server as bytes
			let mut message = String::from_str("1");
			message.push_str(String::from_char(1, (user.len() as u8) as char).as_slice());
			message.push_str(user.as_slice());
			message.push_str(String::from_char(1, (pass.len() as u8) as char).as_slice());
			message.push_str(pass.as_slice());
			
			write_message(message, stream.clone());

			// block and read op code
			stream.read(&mut buf);
			op = buf[0] as char;

		    // if the op code is '3' remove message from stream and check the op code again
		    while op == '3' {
		    	stream.read(&mut buf);
		    	for n in range(0u, buf[0] as uint) { stream.read(&mut buf); }
		    	stream.read(&mut buf);
		    	op = buf[0] as char;
		    }

		    // if op code is 1 login is successful and the client is moved on to the message loop
		    if op == '1' {

		    	let input = Arc::new(Mutex::new("".to_string()));

		    	// cloned resources used in both sendinng and receiving
		    	let mut inputStream = stream.clone();
		    	let mut outputStream = stream.clone();
		    	let mut in_access = UI_access.clone();
		    	let mut out_access = UI_access.clone();
		    	let mut in_messages = messages.clone();
		    	let mut out_messages = messages.clone();
		    	let mut in_input = input.clone();
		    	let mut out_input = input.clone();

		    	

		    	// start termbox user interface
		    	rustbox::init();

				// add info to message output
				{
					let mut messages = messages.lock();
					messages.push("Welcome to the chatroom!".to_string());
					messages.push("There is a 127 character limit on messages.".to_string());
					messages.push("Press the escape key to exit.".to_string());
					messages.push("".to_string());
					ui_print_messages((*messages).iter());
				}

				rustbox::present();

		    	// spawn input task
		    	spawn(move || {
		    		let mut test = 1u16;

			    	// user input loop
			    	loop {

		    			// poll for a termbox event
		    			match rustbox::poll_event() {

				        	// if the event is a keypress, account for thekey in the UI
				        	rustbox::Event::KeyEvent(bleh, control, ch) => {

				            	// aquire access to the UI
				            	in_access.acquire();
				            	{
					            	// if espcape key
					            	if control == 27u16 {
					            		let mut message = "0".to_string();
					            		write_message(message, inputStream.clone());
					            		break;
					            	}

					            	// if enter key
					            	if control == 13u16 {
					            		let mut in_input = in_input.lock();
					            		let mut message = "2".to_string();

					            		message.push_str(String::from_char(1, 
					            			((*in_input).len() as u8) as char).as_slice());
					            		message.push_str((*in_input).as_slice());
					            		write_message(message, inputStream.clone());
					            		(*in_input) = "".to_string();

					            		rustbox::clear();
					            		rustbox::present();
					            	}

					            	// if backspace key
					            	if control == 127u16 {
					            		let mut in_input = in_input.lock();
					            		let messages = in_messages.lock();
					            		(*in_input).pop();
					            		let input = (*in_input).as_slice();

					            		rustbox::clear();
					            		ui_print_messages((*messages).iter());
					            		rustbox::print(0, rustbox::height() - 1, 
					            			Style::Bold, Color::White, Color::Default, 
					            			message_fit_to_window(input.to_string(), rustbox::width()));
					            		rustbox::present();
					            	}

					            	// if spacebar
					            	if control == 32u16 {
					            		let mut in_input = in_input.lock();
					            		let messages = in_messages.lock();

					            		if (*in_input).len() < 127 {
					            			(*in_input).push(' ');
					            		}
					            		let input = (*in_input).as_slice();

					            		rustbox::clear();
					            		ui_print_messages((*messages).iter());
					            		rustbox::print(0, rustbox::height() - 1, 
					            			Style::Bold, Color::White, Color::Default, 
					            			message_fit_to_window(input.to_string(), rustbox::width()));
					            		rustbox::present();
					            	}

					            	// else a character
					            	else {
					            		match char::from_u32(ch) {
					            			Some('\x00') => {},
					            			Some(c) => {
					            				let mut in_input = in_input.lock();
					            				let messages = in_messages.lock();

					            				if (*in_input).len() < 127 {
					            					(*in_input).push(c);
					            				}
					            				let input = (*in_input).as_slice();

					            				rustbox::clear();
					            				ui_print_messages((*messages).iter());
					            				rustbox::print(0, rustbox::height() - 1, 
					            					Style::Bold, Color::White, Color::Default, 
					            					message_fit_to_window(input.to_string(), rustbox::width()));
					            				rustbox::present();
					            			}
					            			_ => {}
					            		}
					            	}
					            }
					            in_access.release();
					        },
					        _ => {}
					    }
					}
					rustbox::shutdown();

					let mut message = "3".to_string();
					write_message(message, inputStream.clone());
					inputStream.close_read();

					drop(inputStream);
				});

				// start output loop
				loop {
					let mut text = "".to_string();
					let result = outputStream.read(&mut buf);

		    		// match the result to account for the stream being closed 
		    		match result {

						// the initial result exists, read it into the text string
						Ok(result) => {
							let mut op = buf[0] as char;
							if op == '3' {
								outputStream.read(&mut buf);
								for n in range(0u, buf[0] as uint) {
									stream.read(&mut buf);
									text.push(buf[0] as char);
								}
								outputStream.read(&mut buf);
								for n in range(0u, buf[0] as uint) {
									stream.read(&mut buf);
									text.push(buf[0] as char);
								}
							}
							else if op == '4' { 
								rustbox::clear();
								rustbox::print(0, 1, Style::Bold, Color::White, Color::Default,
									"The server has been closed, please exit.".to_string());
								rustbox::present();
								break;
							}
							else { break; }
						}

						// if the result is an error, exit the input loop
						Err(e) => { break; }
					}

					// add the message to the messages vector and refresh the UI 
					let mut messages = out_messages.lock();
					if (*messages).len() < rustbox::height() - 2 {
						(*messages).push(text);
					}
					else {
						(*messages).remove(0);
						(*messages).push(text);
					}

					// aquire access to the UI
					out_access.acquire();
					{
						rustbox::clear();
						let mut out_input = out_input.lock();
						let input = (*out_input).as_slice();
						rustbox::print(0, rustbox::height() - 1, Style::Bold, Color::White, Color::Default, 
							message_fit_to_window(input.to_string(), rustbox::width()));
						ui_print_messages((*messages).iter());
						rustbox::present();
					}
					out_access.release();
				}

			}

		    // if op code is 2 login is failed and the client exits
		    else if op == '2' {
		    	drop(stream);
		    	println!("exit: login failed");
		    }
		    else { println!("invalid op code"); drop(stream); }
		},
		Err(e) => { println!("exit: cannot connect to server"); }
	}
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

// print all messages in the iterator to the UI 
fn ui_print_messages(mut messages: core::slice::Items<String>) {
	let mut i = 0;
	for s in messages {
		rustbox::print(0, i, Style::Bold, Color::White, Color::Default, s.as_slice().to_string());
		i = i+1;
	}
}

// return a String with letters before the width trimmed off
fn message_fit_to_window(message: String, width: uint) -> String {
	let mut new_message = "".to_string();
	let length = message.len();
	let mut i = 0;
	if length < width+1 {
		return message;
	}
	else {
		for ch in message.chars() {
			if i > length-width-1 {
				new_message.push(ch);
			}
			i = i+1;
		}
		return new_message;
	}
}

// removes newline characters for both windows and linux
fn remove_end_newline_char(message : String) -> String {
	let mut new_message = message.to_string();
	let mut last_char = new_message.pop().unwrap();
	while last_char == 13 as char || last_char == 10 as char {
		last_char = new_message.pop().unwrap();
	}
	new_message.push_str(String::from_char(1, last_char).as_slice());
	return new_message;
}