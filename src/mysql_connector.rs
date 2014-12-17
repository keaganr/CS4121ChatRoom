/**
* mysq_connector.rs
* 
* Holds methods used to access MySQL database
* 
* CS4121 Fall 2014
* Rust Chat Room Project
*/
extern crate mysql;
extern crate time;

// Includes
use time::Timespec;
use mysql::conn::{MyOpts};
use mysql::conn::pool::{MyPool};
use mysql::value::{from_value};
use std::default::{Default};

use std::vec;


// Basic structure representing a single user
struct User {
	username: String,
	password: String
}


// Basic structure representing a message
pub struct Message {
	pub userid: int,
	pub message: String,
	pub time_sent: Timespec
}


/**
* Initializes the db connection 
* Returns the pool used to query
* a close of pool must be passed to all functions in mysql_connector. 
* Should probably find a better way to do this.
*/
pub fn init_db_conn() -> mysql::conn::pool::MyPool {
	println!("connecting...");
	let opts = MyOpts{user: Some("rustuser".to_string()), pass: Some("rustuser".to_string()), tcp_addr: Some("www.keaganrasmussen.com".to_string()), ..Default::default()};
	let pool = MyPool::new(opts).unwrap();
	let _ = pool.query("USE Chat_Room");	
	println!("connected to database");
	return pool;
}


/**
* Attempts to authenticate user credentials against database entries
*/
#[allow(unused_variable)]
pub fn authenticate(pool: mysql::conn::pool::MyPool, username: String, password: String) -> bool {

	let statement = "SELECT * FROM user WHERE username='".to_string() + username.as_slice() + "' and password='" + password.as_slice() + "';";

	let mut result = pool.query(statement.as_slice()).unwrap();

	let mut found_flag = true;
	if (result.next() == None) {
		found_flag = false;
	}

	// return credential match results
	return found_flag;
}


/**
* Creates a new user entry in the database with given credentials
*/
#[allow(unused_must_use)]
pub fn add_user(pool: mysql::conn::pool::MyPool, username: String, password: String) {

	let new_user = User {
		username: username,
		password: password
	};

	// TODO: don't allow duplicate usernames

	pool.prepare("INSERT INTO user VALUES (null, ?, ?);")
	.and_then(|mut stmt| {
		stmt.execute(&[&new_user.username, &new_user.password]).and(Ok(()))
	});

}


/**
 * Log a message to the database
 */
 pub fn store_message(pool: mysql::conn::pool::MyPool, new_message: Message) {
 	println!("Message: {}", new_message.message);

 	pool.prepare("INSERT INTO messages VALUES(null, ?, ?, null);")
 	.and_then(|mut stmt| {
 		stmt.execute(&[&new_message.userid, &new_message.message]).and(Ok(()))
 	});
 }

/**
 * Get userid from username
 */
 pub fn get_uid(pool: mysql::conn::pool::MyPool, username: String)  -> int {

 	// // create sql statement
 	let statement = "SELECT * FROM user WHERE username='" + username + "';";

 	// // execute and parse through query to find matching uid
 	let mut found_id : int = 0;

 	let mut res = pool.query(statement.as_slice()).unwrap();
 	let row = res.next().unwrap().unwrap();
 	found_id = from_value(&row[0]);

 	return found_id;
 }

 /**
  * Get $count number of latest messages
  */
  pub fn get_recent_messages(pool: mysql::conn::pool::MyPool, count: int) -> Vec<Message> {
  	//select * from messages order by time limit 3;

 	// create sql statement
 	// let statement = "SELECT * FROM messages ORDER BY time LIMIT ".to_string() + count.to_string() + ";".to_string();

 	// // build list of messages
 	let mut msg_vec : Vec<Message> = Vec::new();
 	return msg_vec;
 	// let mut res = pool.query(statement.as_slice()).unwrap();
 	// let row = res.next().unwrap().unwrap();
 	// let mut new_message = Message { 
 	// 	userid: from_value(&row[1]),
 	// 	message: from_value(&row[2]),
 	// 	time_sent: from_value(&row[3])
 	// };
 	// println!("{}", new_message.message);

 	// println("{}", row.next());

 	// let _ = pool.prepare(statement.as_slice())
 	// .and_then(|mut stmt| {
 	// 	for row in &mut stmt.execute([]) {
 	// 		let row = row.unwrap();

 	// 		// create individual message
 	// 		let mut new_message = Message {
 	// 			userid: from_value(&row[1]),
 	// 			message: from_value(&row[2]),
 	// 			time_sent: from_value(&row[3])
 	// 		};
 	// 		msg_vec.push(new_message);
 	// 	}
 	// 	Ok(())
 	// });

 	// return message vector (list)
 	// return msg_vec;
 }








































