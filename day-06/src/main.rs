use std::fs;
use std::collections::HashMap;


const SEQ_SIZE: usize = 4; //14 for part 2


fn main() {

	// Read the file as a string
	let file: String = fs::read_to_string("input")
		.expect("Should have been able to read the file");

	// Starting from the Nth character, check if any repeats in the string of that size
	for i in SEQ_SIZE..file.len() {

		// Get the string range of the correct size
		let range: &str = &file[i-SEQ_SIZE..i];

		// Make a hash table to store repeats 
		let mut repeats: HashMap<char,usize> = HashMap::new();

		// Iterate through the text, adding all of the values the has table
		let mut repeated: bool = false;
		
		for char in range.chars() {

			// If the value has already been added, then a repeat occurred
			if repeats.contains_key(&char) {
				repeated = true;
			}
		
			repeats.entry(char).or_insert(1);
		}

		if !repeated {
			println!("The packet sequence starts at {}", i);
			return; //no more checks needed
		}
	}
}
