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
	// let mut auth = mysql_connector::authenticate(pool.clone(), "username".to_string(), "password".to_string());
	// println!("{}",auth);

	// authenticate user fail example
	// auth = mysql_connector::authenticate(pool.clone(), "useme".to_string(), "password".to_string());
	// println!("{}",auth);

	// store message
	// let msg = mysql_connector::Message {
	// 	userid: mysql_connector::get_uid(pool.clone(), "username".to_string()),
	// 	message: "Sent from user 18?".to_string(),
	// 	time_sent: time::get_time()
	// };

	// mysql_connector::store_message(pool.clone(), msg);

	let msg_vec : Vec<mysql_connector::Message> = mysql_connector::get_recent_messages(pool.clone(), 40);
	for msg in msg_vec.iter() {
		println!("msg = {}", msg.userid);
	}

 }