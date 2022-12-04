use std::fs;

/*
 * Type =	opp/you
 * Rock =	A/X
 * Paper =	B/Y
 * Scizor =	C/Z 
 *
 * Scoring (per round):
 * Win +=	6
 * Draw +=	3
 * Lose +=	0
 *
 * Rock +=	1
 * Paper +=	2
 * Scizor+=	3 
*/

fn scoring_table (x : &str) -> usize {
	match x {
		"AX" => 3,
		"AY" => 6,
		"AZ" => 0,
		"BX" => 0,
		"BY" => 3,
		"BZ" => 6,
		"CX" => 6,
		"CY" => 0,
		"CZ" => 3,

		"X" => 1,
		"Y" => 2,
		"Z" => 3,
		_ => 0,
	}
}


fn main() {

	// Read the file as a string
	let file: String = fs::read_to_string("input")
		.expect("Should have been able to read the file");

	let rounds: Vec<&str> = file.split('\n').collect();
	let mut total_score: usize = 0; 

	for round in rounds {
		if round.is_empty() { continue; } // Ignore empty values
		total_score += calculate_score(&round[0..1], &round[2..3]);
	}

	println!("{}", total_score);
}

fn calculate_score (opp_choice: &str, you_choice: &str) -> usize {

	let mut score: usize = 0;


    let matchup : String = format!("{}{}", opp_choice, you_choice);

	score += scoring_table(&matchup);
	score += scoring_table(you_choice);

	score
}
