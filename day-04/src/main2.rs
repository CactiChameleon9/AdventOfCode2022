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
		let range1 = range1min..range1max + 1;
		let range1_2 = range1min..range1max + 1; //2nd version because of moved range1

		// Get the string vector of the range, then get the min and max (ints) the get the final range
		let range2str: Vec<&str> = pair.split(',').collect::<Vec<&str>>()[1].split('-').collect();
		let range2min: usize = range2str[0].parse().unwrap();
		let range2max: usize = range2str[1].parse().unwrap();
		let range2 = range2min..range2max + 1;

		// Check if any values within the range are in the other range (inefficent sadly)
		let mut overlap = false;
		for i in range1{
			if overlap { break ;}
			
			if range2.contains(&i) {
				overlap = true;
			}
		}
		
		for i in range2{
			if overlap { break ;}

			if range1_2.contains(&i) {
				overlap = true;
			}
		}

		if overlap { overlap_pairs += 1; }
	}

	println!("The number of overlapping pairs are: {overlap_pairs}");
	
}
