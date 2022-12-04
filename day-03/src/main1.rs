use std::fs;
use std::collections::HashMap;


pub fn main() {

	// Read the file as a string
	let file: String = fs::read_to_string("input")
		.expect("Should have been able to read the file");

	let mut total_priority: usize = 0; 

	// Split into array of bags
	let bags: Vec<&str> = file.split('\n').collect();
	for bag in bags {
		if bag.is_empty() { continue; } // Ignore empty values

		// Split the bag into its 2 compartments
		let len: usize = bag.len();
		let compartment1 = &bag[0..len/2];
		let compartment2 = &bag[len/2..len];

		// Find the repeated values and add to the total priority
		let repeated: char = find_repeated(compartment1, compartment2);
		total_priority += calculate_priority(repeated);
	}

	// Print the final total
	println!("The total priotity is: {total_priority}")
}

fn find_repeated(text1: &str, text2: &str) -> char {

	// Make a hash table to store repeats 
	let mut repeats: HashMap<char,usize> = HashMap::new();

	// Iterate through the first text, adding all of the values the has table
	for char in text1.chars() {
		repeats.entry(char).or_insert(1);
	}

	// Hash all of text2's values until a duplicate is found
	for char in text2.chars() {
		if repeats.contains_key(&char) {
			return char; // return the duplicate
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
