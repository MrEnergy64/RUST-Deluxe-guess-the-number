# RUST guess the number "Deluxe" Version

Binaries:

Linux:
- https://github.com/MrEnergy64/RUST-guess-the-number/tree/main/binaries/LINUX

Windows (run in PowerShell UTF-8):
- https://github.com/MrEnergy64/RUST-guess-the-number/tree/main/binaries/WINDOWS


my first RUST experience

a small "guess the number" game, with some RUST solutions for :

1. Clear screen:
	- print!("{}[2J", 27 as char);
  
2. remove the \r after enter an input:
	- while namen.ends_with('\n') || namen.ends_with('\r') {
		- namen.pop();
    
3. restart the program (like in the good old basic time with a goto style  ....):

	- println!("Spiel neu starten mit neuem Spieler (schreibe 'ja', ansonsten [enter] für gleichen Spieler) ?");
  
  	- io::stdin()
  		- .read_line(&mut neuer)
  		- .expect("Fehler beim Lesen der Zeile");
  	- if neuer.trim() == "ja" {
  		- eingabe_namen();
  	- } else {
  		- eingabe_zahlen(namen);
  	- }

4. use escape sequences for screen position print out:
	- pub fn mv_point(line: usize, col: usize) {
		- print!("\x1b[{};{}H", col, line);
	- } // end of mv_point()

5. create a lib.rs and put some fn into it like:
	- clear_screen()
	- set_color()
	- pause()
	- mv_point()
	
and then import it into the main.rs:
	- mod lib.rs

6. add formatet time and date:
	-fn uhrzeit() {
		- let now: DateTime<Local> = Local::now();
		- println!("	{}", now.format("%a - %e %b %Y  - %T\n"));
	- } // end of uhrzeit()
	
7. repeat a string:

	- fn rahmen() {

		- let str1 = "=";
		- let str2 = "|";
		- lib::set_color("green");
		- println!("\n{}{}{}", str2.repeat(1), str1.repeat(44), str2.repeat(1));
		- lib::set_color("reset");

8. added waiting dots (printout in one line):
	- let warten = [".", ".", "."];
		- for x in 0..3 {
		- print!("{} ", warten[x]);
		- io::stdout().flush().unwrap();
		- lib::pause(800);
	- }
	



![RUST-gues-the-number](https://user-images.githubusercontent.com/58075655/201479113-bc953fd8-2fd9-4514-90a1-05be0124e466.gif)


	
