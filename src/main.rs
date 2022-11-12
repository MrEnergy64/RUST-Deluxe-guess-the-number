
/// RUST guess-the-number
/// a little game to demostrate
/// first steps with RUS programming
/// language and shows some programming
/// how to do's

/// Version 1.5 by Norman Wöske



use std::io::{self, Write};
use rand::Rng;
use chrono::prelude::*;
extern crate rand;
mod lib;


fn main() {
    willkommen();
} //end of main()

fn uhrzeit() {
	let now: DateTime<Local> = Local::now();
	println!("	{}", now.format("%a - %e %b %Y  - %T"));

} // end of uhrzeit()

fn rahmen() {
	let str1 = "=";
	let str2 = "|";
	lib::set_color("green");
	println!("\n{}{}{}\n", str2.repeat(1), str1.repeat(44), str2.repeat(1));
	lib::set_color("reset");

}  // end of rahmen()

fn warten() {
	let warten = [".", ".", "."];
		for x in 0..3 {
			print!("{} ", warten[x]);
			io::stdout().flush().unwrap();
			lib::pause(800);
		}

} // end of warten()

fn willkommen() {
	lib::clear_screen();
	lib::mv_point(0,0);

	const WILLKOMMEN: &str = " 	
   	*****************************
   	*    W I L L K O M M E N    *
   	*                           *
   	* (c) Norman Wöske     V1.5 *
   	*****************************";
	
	rahmen();
	lib::set_color("yellow");
	println!("{}", WILLKOMMEN);
	lib::set_color("reset");
	uhrzeit();
	
	eingabe_namen();

} // end of willkommen()

fn eingabe_namen() {
	let mut namen = String::new();
	rahmen();
	print!("     Hallo, \x1b[94mSpieler\x1b[0m! Wie ist dein Name?\n\n     Name: ");
	
	let _ = io::stdout().flush();	
	 io::stdin()
		.read_line(&mut namen)
		.expect("Fehler beim Lesen der Zeile");
	
	while namen.ends_with('\n') || namen.ends_with('\r') {
		namen.pop();
	}

	gaming_time(namen);

} // end of eingabe_namen()

fn gaming_time(namen: String) {
	lib::clear_screen();
	lib::mv_point(0,0);


	const GAMING: &str = " 	
   	*****************************
   	*    G A M I N G  T I M E   *
   	*                           *
   	*                           *
   	*****************************";

	rahmen();
	lib::set_color("yellow");
	println!("{}", GAMING);
	lib::set_color("reset");
	uhrzeit();
	rahmen();
 	println!("     Hallo \x1b[94m{}\x1b[0m, lass uns ein Spiel spielen...\n", namen);
 	
	print!("     Bei 3 geht es los.... ");
	let warten = ["1", "2", "3"];
	for x in 0..3 {
		print!("{} ", warten[x]);
		io::stdout().flush().unwrap();
		lib::pause(1500);
	}
	

	 zahlenspiel(namen);
	
} //end of gaming_time()

fn zahlenspiel(namen: String) {
	let zaehler: i32 = 1;

	let secret_number = rand::thread_rng().gen_range(1..101);
	
	let secret_number2 = secret_number.to_string();

	zahlen_eingabe(secret_number2, namen, zaehler);

} // end of zahlenspiel()

fn zahlen_eingabe(secret_number2: String, namen: String, zaehler: i32) {
	lib::clear_screen();
	lib::mv_point(0,0);
	
	const RATEN: &str = "
	*****************************
 	*       Rate die Zahl!      *
 	*                           *
	*                           *
 	*****************************";

 	rahmen();
 	lib::set_color("yellow");
	println!("{}", RATEN);
	lib::set_color("reset");
	uhrzeit();
	lib::set_color("magenta");
	println!("\n            Rateversuch {} von 10...", zaehler);
	lib::set_color("reset");
	rahmen();
	let mut guess = String::new();
 	print!("\x1b[94m{}\x1b[0m, bitte gib deine Zahl zwischen 1-100 ein.\n\n           => : ", namen);
	let _ = io::stdout().flush();
 	io::stdin()
		.read_line(&mut guess)
	 	.expect("Fehler beim Lesen der Zeile");
	
	while guess.ends_with('\n') || guess.ends_with('\r') {
		guess.pop();
	}

	auswertung(secret_number2, guess, namen, zaehler);

} // end of zahlen_eingabe()

fn auswertung(secret_number2: String, guess: String, namen: String, zaehler: i32) {
	lib::clear_screen();
	lib::mv_point(0,0);

	const AUSWERTUNG: &str = "
	*****************************
 	*    A U S W E R T U N G    *
 	*                           *
	*                           *
 	*****************************";

	rahmen();
 	lib::set_color("yellow");
	println!("{}", AUSWERTUNG);
	lib::set_color("reset");
	uhrzeit();
	rahmen();

	if zaehler >= 10 {
		looser(secret_number2, guess, namen);
	} else {
		let secret_int: u32 = secret_number2
		.trim()
		.parse()
		.expect("Wanted a number");

		let guess_int: u32 = guess
		.trim()
		.parse()
		.expect("Wanted a number"); 
	
		if guess_int > secret_int {
			print!("\nDeine Zahl {} ist größer als die Geheimzahl, rate weiter ", guess);
			warten();
			let zaehler = zaehler + 1;
			zahlen_eingabe(secret_number2, namen, zaehler);
		} else if  guess_int < secret_int {
			print!("\nDeine Zahl {} ist kleiner als die Geheimzahl, rate weiter ", guess);
			warten();
			let zaehler = zaehler + 1;
			zahlen_eingabe(secret_number2, namen, zaehler);
		} else {
			winner(secret_number2, namen, zaehler);
		}
	}	
} // end of auswertung()

fn winner(secret_number2: String, namen: String, zaehler: i32) {
	lib::clear_screen();
	lib::mv_point(0,0);

	const WINNER: &str = "
	*****************************
	*    !!   GEWINNER   !!     *
	*                           *
	*           :-)             *
	*****************************";

	rahmen();
	lib::set_color("yellow");
	println!("{}", WINNER);
	lib::set_color("reset");
	uhrzeit();
	rahmen();
	
	println!("\x1B[3m   *** Juhuu, \x1b[94m{}\x1b[0m\x1B[3m, du hast gewonnen!! ***\n\x1b[0m", namen);
	println!("Die zu erratende Zahl war: \x1b[93m{}\x1b[0m, und in \x1b[93m{}\x1b[0m Versuch(e) erraten.\n", secret_number2, zaehler);
	lib::pause(1000);

	nochmal(namen);

} //end of winner

fn looser(secret_number2: String, x: String, namen: String) {
	lib::clear_screen();
	lib::mv_point(0,0);

	const LOSER: &str = "
	*****************************
	*    !!   VERLOREN   !!     *
	*                           *
	*           :-(             *
	*****************************";

	rahmen();
	lib::set_color("cyan");
	println!("{}", LOSER);
	lib::set_color("reset");
	uhrzeit();
	rahmen();

	println!("\x1B[3m >> Schade, \x1b[94m{}\x1b[0m\x1B[3m, du hast leider verloren. <<\n\x1b[0m", namen);
	println!("Die zu erratende Zahl war: \x1b[93m{}\x1b[0m, deine letzte Zahl war: \x1b[93m{}\x1b[0m\n", secret_number2, x);
	lib::pause(1000);

	nochmal(namen);

} //end of looser()

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
			gaming_time(namen);
		}
   	}
} //end of nochmal()
