# guess the number

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


