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
// use time::Timespec;
use mysql::conn::{MyOpts};
use mysql::conn::pool::{MyPool};
// use mysql::value::{from_value};
use std::default::{Default};

// Basic structure representing a single user
struct User {
	username: String,
	password: String
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

	// build string
	let statement = "SELECT * FROM user WHERE username='".to_string() + username + "' and password='".to_string() + password + "';".to_string();

	// execute and parse through query
	let mut found_flag = false;
	let _ = pool.prepare(statement.as_slice())
	.and_then(|mut stmt| {
		for row in &mut stmt.execute([]) {
			// row exists: means a db row match was found
			found_flag = true;
		}
		Ok(())
	});

	// return credential match results
	return found_flag;
}

/**
* Creates a new user entry in the database with given credentials
*/
#[allow(unused_must_use)]
pub fn add_user(username: String, password: String) {
	let pool = init_db_conn();

	let new_user = User {
		username: username,
		password: password
	};

	pool.prepare("INSERT INTO user VALUES (null, ?, ?);")
	.and_then(|mut stmt| {
		stmt.execute(&[&new_user.username, &new_user.password]).and(Ok(()))
	});

}

