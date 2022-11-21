
/// RUST guess-the-number
/// a little game to demostrate
/// first steps with RUS programming
/// language and shows some programming
/// how to do's

/// Version 1.9 by Norman Wöske



use std::io::{self, Write};
use std::io::stdout;
use std::fs::OpenOptions;
use std::fs;
use rand::Rng;
use chrono::prelude::*;
use std::{thread, time};
use crossterm::{
	execute, queue,
	style::{self, Stylize, Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
	cursor, terminal, Result
};


extern crate rand;

fn main() {

    willkommen().ok();
    
} //end of main()


pub fn set_color(c: &str) {
        match c {
                "red"           => print!("\x1b[31m"),
                "green"         => print!("\x1b[32m"),
                "yellow"        => print!("\x1b[33m"),
                "cyan"          => print!("\x1b[36m"),
                "magenta"       => print!("\x1b[35m"),
                "reset"         => print!("\x1b[0m"),
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

fn uhrzeit(wo: u16) -> Result<()> {

	let now: DateTime<Local> = Local::now();
	
	execute!(
		stdout(),
		Print("        "),
		SetForegroundColor(Color::Yellow),
		SetBackgroundColor(Color::Blue),
		cursor::MoveTo(20, wo),
		Print(now.format("%a - %e %b %Y  - %T")),
		ResetColor
	)?;

	Ok(())
		
} // end of uhrzeit()

fn rahmen2() -> Result<()> {

	let test = "|===========================================|";
	let mut stdout = stdout();


	execute!(stdout, terminal::Clear(terminal::ClearType::All))?;

  	for y in 0..26 {
  		for x in 0..72 {
  	    	if (y == 0 || y == 26 - 1) || (x == 0 || x == 72 - 1) {
  	    		queue!(stdout, cursor::MoveTo(x,y), style::PrintStyledContent( "█".magenta()))?;
  	    	 }
         }
     }
    execute!(stdout, cursor::MoveTo(12, 2), style::PrintStyledContent( test.green()))?;
	stdout.flush()?;
	Ok(())
}

fn rahmen(wo: u16) -> Result<()> {
	let test = "|===========================================|";
	let mut stdout = stdout();
	
	execute!(stdout, cursor::MoveTo(12, wo), style::PrintStyledContent( test.green()))?;

	stdout.flush()?;
	Ok(())
		
}  // end of rahmen()

fn warten() {
	let warten = [".", ".", "."];
		for x in 0..3 {
			print!("{} ", warten[x]);
			io::stdout().flush().unwrap();
			pause(1000);
		}

} // end of warten()

fn willkommen() -> Result<()> {

	let mut stdout = stdout();
	
	rahmen2().ok();
	
	execute!(stdout, cursor::MoveTo(20, 3), style::PrintStyledContent( "*****************************".yellow()))?;
	execute!(stdout, cursor::MoveTo(20, 4), style::PrintStyledContent( "*    W I L L K O M M E N    *".yellow()))?;	
	execute!(stdout, cursor::MoveTo(20, 5), style::PrintStyledContent( "*                           *".yellow()))?;
	execute!(stdout, cursor::MoveTo(20, 6), style::PrintStyledContent( "* (c) Norman Wöske     V1.9 *".yellow()))?;
	execute!(stdout, cursor::MoveTo(20, 7), style::PrintStyledContent( "*****************************".yellow()))?;
	uhrzeit(8).ok();

	eingabe_namen().ok();

	stdout.flush()?;
	Ok(())

} // end of willkommen()

fn eingabe_namen() -> Result<()> {
	let mut namen = String::new();
	let mut stdout = stdout();

	rahmen(10).ok();
	execute!(stdout, cursor::MoveTo(16, 12), style::PrintStyledContent( "Hallo, \x1b[94mSpieler\x1b[0m! Wie ist dein Name?".yellow()))?;
	execute!(stdout, cursor::MoveTo(16, 14), style::PrintStyledContent( "Name: ".yellow()))?;
	
	let _ = io::stdout().flush();	
	io::stdin()
		.read_line(&mut namen)
		.expect("Fehler beim Lesen der Zeile");
	
	while namen.ends_with('\n') || namen.ends_with('\r') {
		namen.pop();
	}


	execute!(stdout, cursor::MoveTo(16, 16), style::PrintStyledContent( "Schön das du hier bist ".yellow()))?;
	println!("\x1b[94m{}\x1b[0m.", namen);
	execute!(stdout, cursor::MoveTo(8, 18), style::PrintStyledContent( "Möchtest Du dein Log-Datei sehen (1=ja, [enter]=nein) ? ".yellow()))?;
	
	let _ = io::stdout().flush();	
	let mut protokoll = String::new();
	io::stdin()
		.read_line(&mut protokoll)
		.expect("Fehler beim Lesen der Zeile");
	
	while protokoll.ends_with('\n') || protokoll.ends_with('\r') {
			protokoll.pop();
	}

	if protokoll == "1" {
		let fp = "results.txt";
		let b = std::path::Path::new(fp).exists();

		if b == false {
			execute!(stdout, cursor::MoveTo(4, 20), style::PrintStyledContent( "Keine Protokolldatei Result.txt vorhanden, weiter im Programm..".red()))?;
			pause(3000);
		} else {
			clear_screen();
			uhrzeit(2).ok();
			//mv_point(0,0);
			rahmen(3).ok();
			set_color("green");
    		let file_path = "results.txt";
    		println!("\n\nLade Protokoll-Datei: {}... \n", file_path);
			set_color("green");
    		let contents = fs::read_to_string(file_path)
       			.expect("Etwas ging beim Lesen der Datei schief");
			set_color("yellow");
   		 	println!("Protokoll:\n{contents}\n     weiter in 8 Sekunden....\n");
			set_color("reset");

			pause(8000);
		}

	}

	gaming_time(namen).ok();

	stdout.flush()?;
	Ok(())

} // end of eingabe_namen()

fn gaming_time(namen: String) -> Result<()> {

	let mut stdout = stdout();

	rahmen2().ok();

	execute!(stdout, cursor::MoveTo(20, 3), style::PrintStyledContent( "*****************************".yellow()))?;
	execute!(stdout, cursor::MoveTo(20, 4), style::PrintStyledContent( "*    G A M I N G  T I M E   *".yellow()))?;
	execute!(stdout, cursor::MoveTo(20, 5), style::PrintStyledContent( "*                           *".yellow()))?;
	execute!(stdout, cursor::MoveTo(20, 6), style::PrintStyledContent( "*                           *".yellow()))?;
	execute!(stdout, cursor::MoveTo(20, 7), style::PrintStyledContent( "*****************************".yellow()))?;

	uhrzeit(8).ok();
	rahmen(10).ok();

	execute!(stdout, cursor::MoveTo(16, 12), style::PrintStyledContent( "Hallo, ".yellow()))?;
	println!("\x1b[94m{}\x1b[0m....", namen);
	execute!(stdout, cursor::MoveTo(16, 14), style::PrintStyledContent( "Lass uns ein Spiel spielen....".yellow()))?;
 	
	execute!(stdout, cursor::MoveTo(16, 16), style::PrintStyledContent( "Bei 3 geht es los....  ".yellow()))?;
	let warten = [" 1", " 2", " 3"];
	for x in 0..3 {
		execute!(stdout, cursor::MoveTo(37, 16), style::PrintStyledContent( warten[x].blue()))?;
		io::stdout().flush().unwrap();
		pause(1500);
	}
	

	 zahlenspiel(namen);

	Ok(())
	
} //end of gaming_time()

fn zahlenspiel(namen: String) {
	let zaehler: i32 = 1;

	let secret_number = rand::thread_rng().gen_range(1..101);
	
	let secret_number2 = secret_number.to_string();

	zahlen_eingabe(secret_number2, namen, zaehler).ok();

} // end of zahlenspiel()

fn zahlen_eingabe(secret_number2: String, namen: String, zaehler: i32) -> Result<()> {
	
	let mut stdout = stdout();

	rahmen2().ok();

	execute!(stdout, cursor::MoveTo(20, 3), style::PrintStyledContent( "*****************************".yellow()))?;	
	execute!(stdout, cursor::MoveTo(20, 4), style::PrintStyledContent( "*       Rate die Zahl!      *".yellow()))?;
	execute!(stdout, cursor::MoveTo(20, 5), style::PrintStyledContent( "*                           *".yellow()))?;
	execute!(stdout, cursor::MoveTo(20, 6), style::PrintStyledContent( "*                           *".yellow()))?;
	execute!(stdout, cursor::MoveTo(20, 7), style::PrintStyledContent( "*****************************".yellow()))?;

	uhrzeit(8).ok();

	execute!(stdout, cursor::MoveTo(20, 10), style::PrintStyledContent( "Rateversuch ".blue()))?;
	set_color("yellow");
	println!("{}", zaehler);
	set_color("reset");
	execute!(stdout, cursor::MoveTo(34, 10), style::PrintStyledContent( " von 10...".blue()))?;

	rahmen(12).ok();
	let mut guess = String::new();
	
	execute!(stdout, cursor::MoveTo(14, 14), style::PrintStyledContent( "Bitte gib deine Zahl zwischen 1-100 ein: ".yellow()))?;
		
	let _ = io::stdout().flush();
 	io::stdin()
		.read_line(&mut guess)
	 	.expect("Fehler beim Lesen der Zeile");
	
	while guess.ends_with('\n') || guess.ends_with('\r') {
		guess.pop();
	}

	auswertung(secret_number2, guess, namen, zaehler).ok();

	Ok(())
	
} // end of zahlen_eingabe()

fn auswertung(secret_number2: String, guess: String, namen: String, zaehler: i32) -> Result<()>  {
	
	let mut stdout = stdout();

	rahmen2().ok();

	execute!(stdout, cursor::MoveTo(20, 3), style::PrintStyledContent( "*****************************".yellow()))?;	
	execute!(stdout, cursor::MoveTo(20, 4), style::PrintStyledContent( "*    A U S W E R T U N G    *".yellow()))?;
	execute!(stdout, cursor::MoveTo(20, 5), style::PrintStyledContent( "*                           *".yellow()))?;
	execute!(stdout, cursor::MoveTo(20, 6), style::PrintStyledContent( "*                           *".yellow()))?;
	execute!(stdout, cursor::MoveTo(20, 7), style::PrintStyledContent( "*****************************".yellow()))?;

	uhrzeit(8).ok();
	
	rahmen(10).ok();

	if zaehler >= 10 {
		looser(secret_number2, guess, namen).ok();
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
			execute!(stdout, cursor::MoveTo(10, 12), style::PrintStyledContent( "Die Geheimzahl ist kleiner als:".yellow()))?;
			print!(" {} \x1b[33m=> Rate weiter\x1b[0m ", guess);
			warten();
			let zaehler = zaehler + 1;
			zahlen_eingabe(secret_number2, namen, zaehler).ok();
		} else if  guess_int < secret_int {
			execute!(stdout, cursor::MoveTo(10, 12), style::PrintStyledContent( "Die Geheimzahl ist größer als:".yellow()))?;
			print!(" {} \x1b[33m=> Rate weiter\x1b[0m ", guess);
			warten();
			let zaehler = zaehler + 1;
			zahlen_eingabe(secret_number2, namen, zaehler).ok();
		} else {
			winner(secret_number2, namen, zaehler).ok();
		}
		
	}
	Ok(())	
} // end of auswertung()

fn winner(secret_number2: String, namen: String, zaehler: i32) -> Result<()>  {
	
	let mut stdout = stdout();

	rahmen2().ok();

	execute!(stdout, cursor::MoveTo(20, 3), style::PrintStyledContent( "*****************************".yellow()))?;	
	execute!(stdout, cursor::MoveTo(20, 4), style::PrintStyledContent( "*    !!   GEWINNER   !!     *".yellow()))?;
	execute!(stdout, cursor::MoveTo(20, 5), style::PrintStyledContent( "*                           *".yellow()))?;
	execute!(stdout, cursor::MoveTo(20, 6), style::PrintStyledContent( "*           :-)             *".yellow()))?;
	execute!(stdout, cursor::MoveTo(20, 7), style::PrintStyledContent( "*****************************".yellow()))?;
	
	uhrzeit(8).ok();
	
	rahmen(10).ok();
	
	let now: DateTime<Local> = Local::now();

	let fp = "results.txt";
	let b = std::path::Path::new(fp).exists();

	if b == false {
		let mut w = OpenOptions::new()
			.create_new(true)
			.write(true)
			.append(true)
			.open("results.txt")
			.unwrap();
		if let Err(e) = writeln!(&mut w, "Datum: {}, Name: {}, Gewonnen in {} versuchen.", now.format("%a - %e %b %Y  - %T"), namen, zaehler) {
			eprintln!("Couldn't write to file: {}", e);
		}
	
			
	} else {
		let mut w = OpenOptions::new()
			.write(true)
			.append(true)
			.open("results.txt")
			.unwrap();
		if let Err(e) = writeln!(&mut w, "Datum: {}, Name: {}, Gewonnen in {} versuchen.", now.format("%a - %e %b %Y  - %T"), namen, zaehler) {
			eprintln!("Couldn't write to file: {}", e);
		}
	}
	execute!(stdout, cursor::MoveTo(15, 12), style::PrintStyledContent( "   *** Juhuu, du hast gewonnen!! ***".yellow()))?;
	execute!(stdout, cursor::MoveTo(12, 14), style::PrintStyledContent( "Die zu erratende Zahl war: ".yellow()))?;
	println!("\x1b[36m{}\x1b[0m", secret_number2);
	execute!(stdout, cursor::MoveTo(12, 15), style::PrintStyledContent( "Und wurde von Dir in: ".yellow()))?;
	println!("\x1b[36m{}\x1b[0m", zaehler);
	execute!(stdout, cursor::MoveTo(35, 15), style::PrintStyledContent( " Versuch(e) erraten.".yellow()))?;
	pause(1000);

	nochmal(namen).ok();

	Ok(())
} //end of winner

fn looser(secret_number2: String, x: String, namen: String) -> Result<()> {
	
	let mut stdout = stdout();

	rahmen2().ok();

	execute!(stdout, cursor::MoveTo(20, 3), style::PrintStyledContent( "*****************************".yellow()))?;	
	execute!(stdout, cursor::MoveTo(20, 4), style::PrintStyledContent( "*    !!   VERLOREN   !!     *".yellow()))?;
	execute!(stdout, cursor::MoveTo(20, 5), style::PrintStyledContent( "*                           *".yellow()))?;
	execute!(stdout, cursor::MoveTo(20, 6), style::PrintStyledContent( "*           :-(             *".yellow()))?;
	execute!(stdout, cursor::MoveTo(20, 7), style::PrintStyledContent( "*****************************".yellow()))?;
	
	uhrzeit(8).ok();
	
	rahmen(10).ok();
	
	let now: DateTime<Local> = Local::now();

	let fp = "results.txt";
	let b = std::path::Path::new(fp).exists();

	if b == false {
		let mut w = OpenOptions::new()
			.create_new(true)
			.write(true)
			.append(true)
			.open("results.txt")
			.unwrap();
		if let Err(e) = writeln!(&mut w, "Datum: {}, Name: {}, Verloren!!", now.format("%a - %e %b %Y  - %T"), namen) {
			eprintln!("Couldn't write to file: {}", e);
		}

	} else {
		let mut w = OpenOptions::new()
			.write(true)
			.append(true)
			.open("results.txt")
			.unwrap();
		if let Err(e) = writeln!(&mut w, "Datum: {}, Name: {}, Verloren!!", now.format("%a - %e %b %Y  - %T"), namen) {
			eprintln!("Couldn't write to file: {}", e);
		}
	}
	execute!(stdout, cursor::MoveTo(14, 12), style::PrintStyledContent( " Schade, du hast leider verloren: ".yellow()))?;
	println!("\x1b[36m{}\x1b[0m", namen);
	execute!(stdout, cursor::MoveTo(12, 14), style::PrintStyledContent( "Die zu erratende Zahl war: ".yellow()))?;
	println!("\x1b[36m{}\x1b[0m", secret_number2);
	execute!(stdout, cursor::MoveTo(12, 15), style::PrintStyledContent( "Deine letzte Zahl war: ".yellow()))?;
	println!("\x1b[36m{}\x1b[0m", x);
	pause(1000);

	nochmal(namen).ok();

	Ok(())

} //end of looser()

fn nochmal(namen: String) -> Result<()>  {
	let mut weiter = String::new();
	let mut neuer = String::new();
	let mut stdout = stdout();

	execute!(stdout, cursor::MoveTo(12, 17), style::PrintStyledContent( "Möchtest Du nochmal spielen ".yellow()))?;
	println!("\x1b[94m{}\x1b[0m?", namen);
	execute!(stdout, cursor::MoveTo(2, 19), style::PrintStyledContent( "(schreibe 'nein' für beenden, ansonsten nur [enter] für weiter) ".yellow()))?;
		
	io::stdin()
	    .read_line(&mut weiter)
	    .expect("Fehler beim Lesen der Zeile");
   	
	if weiter.trim() == "nein" {
		clear_screen();
	 	mv_point(0,0);
		println!("\n\x1b[0mSchade, Goodbye \x1b[94m{}\x1b[0m\n", namen);   		   	
   	} else {
		execute!(stdout, cursor::MoveTo(12, 21), style::PrintStyledContent( "Spiel neu starten mit neuem Spieler ?".yellow()))?;
		execute!(stdout, cursor::MoveTo(4, 23), style::PrintStyledContent( "(schreibe 'ja', ansonsten nur [enter] für gleichen Spieler) ? ".yellow()))?;
		io::stdin()
			.read_line(&mut neuer)
	    	.expect("Fehler beim Lesen der Zeile");

		if neuer.trim() == "ja" {
			main();
		} else {
			gaming_time(namen).ok();
		}
   	}
	Ok(())
} //end of nochmal()
