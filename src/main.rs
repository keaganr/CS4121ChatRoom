extern crate mysql_connector;
extern crate mysql;
// extern crate time;

use mysql::conn::pool::{MyPool};
// // use time::Timespec;

// #[allow(unused_must_use)]
fn main() {

	let pool = mysql_connector::init_db_conn();

	let auth  = mysql_connector::authenticate(pool.clone(), "username".to_string(), "password".to_string());
	println!("login ret: {}", auth);


 }