use std::fs;

pub fn main() {

	// Read the file as a string
	let file: String = fs::read_to_string("input")
		.expect("Should have been able to read the file");

	let pairs: Vec<&str> = file.split('\n').collect();

	let mut inside_pairs: usize = 0;

	for pair in pairs{

		if pair.is_empty() { continue; } // Ignore empty values

		// Get the string vector of the range, then get the min and max (ints) the get the final range
		let range1str: Vec<&str> = pair.split(',').collect::<Vec<&str>>()[0].split('-').collect();
		let range1min: usize = range1str[0].parse().unwrap();
		let range1max: usize = range1str[1].parse().unwrap();

		// Get the string vector of the range, then get the min and max (ints) the get the final range
		let range2str: Vec<&str> = pair.split(',').collect::<Vec<&str>>()[1].split('-').collect();
		let range2min: usize = range2str[0].parse().unwrap();
		let range2max: usize = range2str[1].parse().unwrap();

		// If either range contains both the other's min and max, then increase the inside pairs
		if (range1max >= range2max && range1min <= range2min)
		|| (range2max >= range1max && range2min <= range1min) {
			inside_pairs += 1;
		}
	}

	println!("The number of inside pairs are: {inside_pairs}");
	
}
