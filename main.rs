use std::{thread, time, io::{self}};
use rand::Rng;
extern crate rand;

fn main() {
    willkommen();
}

fn willkommen() {
	print!("{}[2J", 27 as char);

	const WILLKOMMEN: &str = " 	
   	\x1b[93m*****************************
   	*    W I L L K O M M E N    *
   	*                           *
   	* (c) Norman Woeske         *
   	*****************************\x1b[0m\n";

	println!("{}", WILLKOMMEN);

	eingabe_namen();
}

fn eingabe_namen() {
	
	let mut namen = String::new();

	println!("     Hallo, \x1b[94mSpieler\x1b[0m! Wie ist dein Name?\n");
	io::stdin()
		.read_line(&mut namen)
		.expect("Fehler beim Lesen der Zeile");
	while namen.ends_with('\n') || namen.ends_with('\r') {
		namen.pop();
	}
	eingabe_zahlen(namen);

}

fn eingabe_zahlen(namen: String) {

	let ten_millis = time::Duration::from_millis(1000);
	let mut guess = String::new();

	print!("{}[2J", 27 as char);

	const GAMING: &str = " 	
   	\x1b[93m*****************************
   	*    G A M I N G  T I M E   *
   	*                           *
   	* (c) Norman Woeske         *
   	*****************************\x1b[0m\n";

	println!("{}", GAMING);

 	println!("\n     Hallo \x1b[94m{}\x1b[0m, lass uns ein Spiel spielen...\n", namen);
 	thread::sleep(ten_millis);

	const RATEN: &str = "
	\x1b[93m*****************************
 	*       Rate die Zahl!      *
 	*                           *
	* (c) Norman Woeske         *
 	*****************************\x1b[0m\n";

 	println!("{}", RATEN);

 	println!("\x1b[94m{}\x1b[0m, bitte gib deine Zahl zwischen 1-10 ein: \n", namen);

 	io::stdin()
		.read_line(&mut guess)
	 	.expect("Fehler beim Lesen der Zeile");

	zahlenspiel(&guess, namen);

}

fn zahlenspiel(x: &str, namen: String) {
	let ten_millis = time::Duration::from_millis(1000);
	let secret_number = rand::thread_rng().gen_range(1..11);
	
	let secret_number2 = secret_number.to_string();
	println!("\nDie zu erratende Zahl war: \x1b[93m{}\x1b[0m, deine Zahl ist: \x1b[93m{}\x1b[0m\n", secret_number2, x);
	if x.trim() == secret_number2 {
		println!("\x1B[3m*** Juhuu, \x1b[94m{}\x1b[0m\x1B[3m, du hast gewonnen!! :-) ***\n\x1b[0m", namen);
		thread::sleep(ten_millis);
		nochmal(namen);
	} else {
		println!("\x1B[3m>> Schade, \x1b[94m{}\x1b[0m\x1B[3m, du hast leider verloren. :-( <<\n\x1b[0m", namen);
		thread::sleep(ten_millis);
		nochmal(namen);
	}
}

fn nochmal(namen: String) {
	let mut weiter = String::new();
	let mut neuer = String::new();

	println!("\n\x1b[94m{}\x1b[0m, möchtest Du nochmal spielen (schreibe 'nein' für beenden, ansonsten [enter] für weiter)?", namen);
	
	io::stdin()
	    .read_line(&mut weiter)
	    .expect("Fehler beim Lesen der Zeile");
   	
	if weiter.trim() == "nein" {
		println!("\n\x1b[0mSchade, Goodbye \x1b[94m{}\x1b[0m\n", namen);   		   	
   	} else {
		println!("Spiel neu starten mit neuem Spieler (schreibe 'ja', ansonsten [enter] für gleichen Spieler) ?");
		io::stdin()
			.read_line(&mut neuer)
	    	.expect("Fehler beim Lesen der Zeile");

		if neuer.trim() == "ja" {
			main();
		} else {
			eingabe_zahlen(namen);
		}
   	}
}
