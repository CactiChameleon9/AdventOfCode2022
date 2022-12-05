use std::fs;

pub fn main() {

	// Read the file as a string
	let file: String = fs::read_to_string("input")
		.expect("Should have been able to read the file");

	let pairs: Vec<&str> = file.split('\n').collect();

	let mut overlap_pairs: usize = 0;

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

		// Check if any values within the range are in the other range
		if (range1max >= range2max && range1min <= range2min) //range2 inside range1
		|| (range2max >= range1max && range2min <= range1min) //range1 inside range2
		|| (range1max >= range2max && range1min <= range2max) //range1 greater than range2, but inside
		|| (range2max >= range1max && range2min <= range1max) { //range2 greater than range1, but inside
			overlap_pairs += 1;
		}
	}

	println!("The number of overlapping pairs are: {overlap_pairs}");
	
}
