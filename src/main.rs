use std::{io::{self}};
use rand::Rng;
extern crate rand;
mod lib;


fn main() {
    willkommen();
} //end of main()


fn willkommen() {
	
	lib::clear_screen();
	lib::mv_point(0,0);

	const WILLKOMMEN: &str = " 	
   	*****************************
   	*    W I L L K O M M E N    *
   	*                           *
   	* (c) Norman Woeske         *
   	*****************************\n";

	lib::set_color("yellow");
	println!("{}", WILLKOMMEN);
	lib::set_color("reset");

	eingabe_namen();

} // end of willkommen()

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

} // end of eingabe_namen()

fn eingabe_zahlen(namen: String) {

	lib::clear_screen();
	lib::mv_point(0,0);

	
	let mut guess = String::new();

	const GAMING: &str = " 	
   	*****************************
   	*    G A M I N G  T I M E   *
   	*                           *
   	* (c) Norman Woeske         *
   	*****************************\n";

	lib::set_color("yellow");
	println!("{}", GAMING);
	lib::set_color("reset");

 	println!("\n     Hallo \x1b[94m{}\x1b[0m, lass uns ein Spiel spielen...\n", namen);
 	lib::pause(2000);

	lib::clear_screen();
	lib::mv_point(0,0);

	const RATEN: &str = "
	*****************************
 	*       Rate die Zahl!      *
 	*                           *
	* (c) Norman Woeske         *
 	*****************************\n";

 	lib::set_color("green");
	println!("{}", RATEN);
	lib::set_color("reset");

 	println!("\x1b[94m{}\x1b[0m, bitte gib deine Zahl zwischen 1-10 ein: \n", namen);

 	io::stdin()
		.read_line(&mut guess)
	 	.expect("Fehler beim Lesen der Zeile");

	zahlenspiel(&guess, namen);

} //end of eingabe_zahlen()

fn zahlenspiel(x: &str, namen: String) {
	
	let secret_number = rand::thread_rng().gen_range(1..11);
	
	let secret_number2 = secret_number.to_string();
	println!("\nDie zu erratende Zahl war: \x1b[93m{}\x1b[0m, deine Zahl ist: \x1b[93m{}\x1b[0m\n", secret_number2, x);
	if x.trim() == secret_number2 {

		lib::clear_screen();
	 	lib::mv_point(0,0);

		const WINNER: &str = "
		*****************************
 		*    !!   GEWINNER   !!     *
 		*                           *
		* (c) Norman Woeske         *
 		*****************************\n";

 		lib::set_color("green");
		println!("{}", WINNER);
		lib::set_color("reset");

		println!("\x1B[3m     *** Juhuu, \x1b[94m{}\x1b[0m\x1B[3m, du hast gewonnen!! :-) ***\n\x1b[0m", namen);
		lib::pause(1000);

		nochmal(namen);
	} else {

		lib::clear_screen();
	 	lib::mv_point(0,0);

		const LOSER: &str = "
		*****************************
 		*    !!   VERLOREN   !!     *
 		*                           *
		* (c) Norman Woeske         *
 		*****************************\n";

 		lib::set_color("cyan");
		println!("{}", LOSER);
		lib::set_color("reset");

		println!("\x1B[3m     >> Schade, \x1b[94m{}\x1b[0m\x1B[3m, du hast leider verloren. :-( <<\n\x1b[0m", namen);
		lib::pause(1000);

		nochmal(namen);
	}
} // end of zahlenspiel()

fn nochmal(namen: String) {
	let mut weiter = String::new();
	let mut neuer = String::new();

	println!("\n\x1b[94m{}\x1b[0m, möchtest Du nochmal spielen (schreibe 'nein' für beenden, ansonsten [enter] für weiter)?", namen);
	
	io::stdin()
	    .read_line(&mut weiter)
	    .expect("Fehler beim Lesen der Zeile");
   	
	if weiter.trim() == "nein" {
		lib::clear_screen();
	 	lib::mv_point(0,0);
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
} //end of nochmal()
