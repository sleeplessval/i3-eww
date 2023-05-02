use std::process::{
	Command,
	exit
};

use i3ipc::{
	I3Connection,
	I3EventListener,
	Subscription,
	event::{Event, ModeEventInfo}
};


pub fn build_widget(connection: &mut I3Connection) {
	let base = "(box :class \"workspace\" :orientation \"h\" :spacing 2 :space-evenly false ";
	let mut output = base.to_string();
	let reply = connection.get_workspaces();
	if reply.is_err() {
		exit(1);
	}
	let workspaces = reply.ok().unwrap();
	for ws in workspaces.workspaces {
		let mut element = String::from("(button :vexpand true :class \"");

		let mut classes = String::from("ws-btn ");
		if ws.focused {
			classes += "focused ";
		}
		if ws.urgent {
			classes += "urgent ";
		}
		element += &(classes + "\" :onclick \"i3-msg workspace ");

		element += &(ws.num.to_string() + "\" \"");
		element += &(ws.num.to_string() + "\")");

		output += &element;
	}

	println!("{})", output);
}

pub fn set_mode(e: ModeEventInfo) {
	let mut cmd = Command::new("eww");
	let mode_str = String::from("WM_MODE=") + &e.change;
	cmd.args(["update", &mode_str]);
	cmd.output().ok();
}

fn main() {
	let mut connection = I3Connection::connect().unwrap();
    build_widget(&mut connection);

	let mut listener = I3EventListener::connect().unwrap();
	let subs = [Subscription::Workspace, Subscription::Mode];
	listener.subscribe(&subs).unwrap();
	for event in listener.listen() {
		match event.unwrap() {
			Event::WorkspaceEvent(_) => build_widget(&mut connection),
			Event::ModeEvent(e) => set_mode(e),
			_ => unreachable!()
		}
	}
}
