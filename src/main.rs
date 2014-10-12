extern crate mysql_connector;
extern crate mysql;

use mysql::conn::pool::{MyPool};

#[allow(unused_must_use)]
fn main() {
	println!("Running from main!");

	// Add user example
	// mysql_connector::add_user("username".to_string(),"password".to_string());

	// establish db connection
	let pool = mysql_connector::init_db_conn();

	// authenticate user example
	let mut auth = mysql_connector::authenticate(pool.clone(), "username".to_string(), "password".to_string());
	println!("{}",auth);

	auth = mysql_connector::authenticate(pool.clone(), "useme".to_string(), "password".to_string());
	println!("{}",auth);

 }