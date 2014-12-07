CS4121ChatRoom
==============

Rust implementation of a basic server / client chat room for CS4121.

rough framework for operations:

	outgoing:

		client:
		COP1	login accepted
		COP2	login rejected

		server:
		SOP0	exit
		SOP1	accept/reject login
		SOP2	send message to all
		SOP3	send history
		SOP4	user announce

	data packets:

		COP1 length byte <user> length byte <pass>
		COP2 <authtoken> <message>

		SOP1 <authtoken>
		SOP2 <user> <message>
		SOP3
