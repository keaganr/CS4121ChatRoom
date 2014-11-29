mysql_connector how to use -


Necessary include statements:

extern crate mysql_connector;
extern crate mysql;
use mysql::conn::pool::{MyPool};


Structs Used:

struct User {
	username: String,
	password: String
}

pub struct Message {
	pub userid: int,
	pub message: String,
	pub time_sent: Timespec
}


Functions:

function calls prefeced by reference to mysql_connector,
ex: mysql_connector::init_db_conn();

init_db_conn()
	Arguments: None
	Returns: mysql::conn::pool::MyPool - A reference to the db connections pool. This method should be called 
		once on server startup, and a clone of the MyPool variable needs to be passed to each subsequent db call ( pool.clone() )

authenticate()
	Arguments:
		mysql::conn::pool::MyPool pool
		String username
		String password
	Returns: bool - Whether or not the given username and password match a database user entry

add_user()
	Arguments:
		mysql::conn::pool::MyPool pool
		String username
		String password
	Returns: None
	Creates a new entry in the database with given user credentials

store_message()
	Arguments:
		mysql::conn::pool::MyPool pool
		Message new_message
	Returns: none
	Stores a message struct in the database for later reference

get_uid()
	Arguments:
		mysql::conn::pool::MyPool pool
		String username
	Returns:
		int - UID corresponding to username given

get_recent_messages()
	Arguments:
		mysql::conn::pool::MyPool pool
		int count
	Returns:
		Vec<Message> - List of $count recent messages stored in the database


