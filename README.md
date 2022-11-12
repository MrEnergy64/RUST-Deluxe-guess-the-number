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

8. added waiting dors (printout in one line):
	- let warten = [".", ".", "."];
		- for x in 0..3 {
		- print!("{} ", warten[x]);
		- io::stdout().flush().unwrap();
		- lib::pause(800);
	- }
	

![guess-the-number-welcome](https://user-images.githubusercontent.com/58075655/200954595-b85cbab8-c640-47a5-8690-b4958023265b.png)

![guess-the-number-zahl](https://user-images.githubusercontent.com/58075655/200954656-1ffdfe28-89d5-4d00-9178-80e136cab23c.png)

![guess-the-number-auswertung](https://user-images.githubusercontent.com/58075655/200954707-d445c9a9-13de-4f23-a5b5-342bf5230363.png)

![guess-the-number-winner](https://user-images.githubusercontent.com/58075655/200954741-fbbb8aed-177e-48f1-b981-6aabf1c8688e.png)

	
	
