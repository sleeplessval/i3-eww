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

///	builds the i3 workspace widget and prints it
pub fn build_widget(connection: &mut I3Connection) {
	//	open base element
	let base = "(box :class \"workspace\" :orientation \"h\" :spacing 2 :space-evenly false ";
	let mut output = base.to_string();

	//	get workspaces from IPC
	let reply = connection.get_workspaces();
	if reply.is_err() {
		exit(1);
	}
	let workspaces = reply.ok().unwrap();

	//	loop to build elements for workspaces
	for ws in workspaces.workspaces {
		//	build classes
		let mut classes = String::from("ws-btn ");
		if ws.focused { classes += "focused "; }
		if ws.urgent { classes += "urgent "; }
		//	build workspace number
		let ws_num = ws.num.to_string();

		//	build element yuck
		let element = format!("(button :vexpand true :class \"{classes}\" :onclick \"i3-msg workspace {ws_num}\" \"{ws_num}\")");
		//	... and add to output
		output += &element;
	}

	//	... and emit!
	println!("{output})");
}

///	issues a command for eww to update the WM_MODE variable on
///	i3 mode change.
pub fn set_mode(e: ModeEventInfo) {
	let mut cmd = Command::new("eww");
	let mode_str = String::from("WM_MODE=") + &e.change;
	cmd.args(["update", &mode_str]);
	cmd.output().ok();
}

fn main() {
	//	open IPC
	let mut connection = I3Connection::connect().unwrap();
	//	build initial widget
    build_widget(&mut connection);

	//	and await workspace and mode events effectively forever
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
