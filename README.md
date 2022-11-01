# my_first_rust
my first RUST experience

a small guess the count game, with some solutions for :
1. Clear screen
  print!("{}[2J", 27 as char);
  
2. remove the \r after enter an input
  while namen.ends_with('\n') || namen.ends_with('\r') {
		namen.pop();
    
2. restart the program (like in the goo old basic time with goto ....)

		println!("Spiel neu starten mit neuem Spieler (schreibe 'ja', ansonsten [enter] für gleichen Spieler) ?");
		io::stdin()
			.read_line(&mut neuer)
	    	.expect("Fehler beim Lesen der Zeile");
		if neuer.trim() == "ja" {
			main();
		} else {
			eingabe_zahlen(namen);
		}
