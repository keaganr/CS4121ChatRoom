extern crate mysql_connector;
extern crate mysql;
extern crate time;

use mysql::conn::pool::{MyPool};
// use time::Timespec;

#[allow(unused_must_use)]
fn main() {
	println!("Running from main!");

	// Add user example
	// mysql_connector::add_user("username".to_string(),"password".to_string());

	// establish db connection
	let pool = mysql_connector::init_db_conn();

	// authenticate user sucess example
	let mut auth = mysql_connector::authenticate(pool.clone(), "username".to_string(), "password".to_string());
	println!("{}",auth);

	// authenticate user fail example
	auth = mysql_connector::authenticate(pool.clone(), "useme".to_string(), "password".to_string());
	println!("{}",auth);

	// store message
	let msg = mysql_connector::Message {
		userid: 0,
		message: "This is a test".to_string(),
		time_sent: time::get_time()
	};

	println!("time: {}", time::get_time().to_string());

	mysql_connector::store_message(pool, msg )

 }