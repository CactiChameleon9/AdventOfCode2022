use std::fs;
use std::collections::HashMap;


pub fn main() {

	// Read the file as a string
	let file: String = fs::read_to_string("input")
		.expect("Should have been able to read the file");

	let mut total_priority: usize = 0; 

	// Split into array of bags
	let bags: Vec<&str> = file.split('\n').collect();

	// Groups of 3 bags, find common item
	let mut i = 0;
	while i < bags.len() - 3 {
	
		// Find the repeated values and add to the total priority
		let repeated: char = find_repeated(bags[i], bags[i+1], bags[i+2]);
		total_priority += calculate_priority(repeated);

		i += 3;
	}

	// Print the final total
	println!("The total badge priotity is: {total_priority}")
}

fn find_repeated(text1: &str, text2: &str, text3: &str) -> char {

	// Make a hash table to store repeats 
	let mut repeats: HashMap<char,usize> = HashMap::new();

	// Iterate through the first text, adding all of the values the has table
	for char in text1.chars() {
		repeats.entry(char).or_insert(1);
	}

	// Hash all of text2's values and if duplicated increase that to 2
	for char in text2.chars() {
		if repeats.contains_key(&char) {
			repeats.insert(char, 2);
		}
	}

	// Hash all of text3's values until the final duplicate duplicate is found, return that
	for char in text3.chars() {
		if repeats.get(&char).copied().unwrap_or(0) == 2 {
			return char
		}
	}
	

	// Nothing if not found
	' '
}

//a-z -> 1-26, A-Z -> 27-52
fn calculate_priority(char : char) -> usize {

	// Flip the ascii round, as capital is normally lower
	let mut	priotity: usize;
	if char.is_lowercase() {
		priotity = char.to_uppercase().last().unwrap() as usize;
	} else {
		priotity = char.to_lowercase().last().unwrap() as usize - 6;
	}

	// Minus 64 from the values so 'a' becomes 1
	priotity -= 64;	
	priotity
}
