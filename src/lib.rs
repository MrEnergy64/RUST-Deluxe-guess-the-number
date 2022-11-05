use std::{thread, time};

pub fn set_color(c: &str) {
	match c {
		"red"		=> print!("\x1b[31m"),
		"green"		=> print!("\x1b[32m"),
		"yellow"	=> print!("\x1b[33m"),
		"cyan"		=> print!("\x1b[36m"),
		"magenta"	=> print!("\x1b[35m"),
		"reset"		=> print!("\x1b[0m"),
		_           => print!("{} is an invalid color ", c),
	}

} // end of set_color()

pub fn clear_screen() {
	print!("{}[2J", 27 as char);

} // end of clear_screen()

pub fn mv_point(line: usize, col: usize) {
	print!("\x1b[{};{}H", col, line);

} // end of mv_point()

pub fn pause(p: u64) {

	let millis = time::Duration::from_millis(p);
	thread::sleep(millis);

} // end of pause()
