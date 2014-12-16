CS4121ChatRoom
==============

Rust implementation of a basic server / client chat room for CS4121.

Server Functionality:

Simply start the server and it will immediately begin waiting for
client connections. Once started the server can be terminated by
entering "exit". However, this only terminates the main process
and notifies clients to exit. It will automatically exit wen they do.

Client Functionality:

With the server started, starting the client will take the user
directly to login. Upon entering allowed credentials, the chat room's
user interface is started. From here the client application allows
the user to enter and receive messages from and to all other running
clients. Pressing the escape key exits the client application.

Hardcoded Defaults:

the server and client are both set to the ip address 127.0.0.1 and
port 8080.

there are three default users set up that may be used multiple times
at once.

	these users are:

	user: user 1 
	password: pass

	user: user 2 
	password: pass

	user: user 3
	password: pass