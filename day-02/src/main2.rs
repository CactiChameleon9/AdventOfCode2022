use std::fs;

/*
 * Type:
 * Rock =	A
 * Paper =	B
 * Scizor =	C 
 * 
 * Aim:
 * Lose =	X
 * Draw =	Y
 * Win =	Z
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
		"AA" => 3,
		"AB" => 6,
		"AC" => 0,
		"BA" => 0,
		"BB" => 3,
		"BC" => 6,
		"CA" => 6,
		"CB" => 0,
		"CC" => 3,

		"A" => 1,
		"B" => 2,
		"C" => 3,
		_ => 0,
	}
}

fn aim_table (x : &str) -> &str {
	match x {
		"AX" => "C",
		"AY" => "A",
		"AZ" => "B",
		"BX" => "A",
		"BY" => "B",
		"BZ" => "C",
		"CX" => "B",
		"CY" => "C",
		"CZ" => "A",
		_ => "",
	}
}


pub fn main() {

	// Read the file as a string
	let file: String = fs::read_to_string("input")
		.expect("Should have been able to read the file");

	// Split into array of each round
	let rounds: Vec<&str> = file.split('\n').collect();
	let mut total_score: usize = 0; 

	// For each round, increase the total score by the calculated score
	for round in rounds {
		if round.is_empty() { continue; } // Ignore empty values
		total_score += calculate_score(&round[0..1], &round[2..3]);
	}

	// Print the final message
	println!("Your informed calculated score is {}", total_score);
}

// This function calulates the score for each using a simple lookup table
fn calculate_score (opp_choice: &str, you_aim: &str) -> usize {

	let mut score: usize = 0;

	// concatenate the aim value and find out what choice is needed
    let aim : String = format!("{}{}", opp_choice, you_aim);
	let you_choice = aim_table(&aim);

	// concatenate the matchup and find out how many point awarded
    let matchup : String = format!("{}{}", opp_choice, you_choice);

	score += scoring_table(&matchup);
	score += scoring_table(you_choice);

	score
}
