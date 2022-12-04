use std::fs;

fn main() {

	// Read the file as a string
	let file: String = fs::read_to_string("input")
        .expect("Should have been able to read the file"); 

	// Split that string up by the double new lines that separate Calories
	let split_file: Vec<&str> = file.split("\n\n").collect();
	// Make a new vairable for the max Calories
	let mut max_cals = vec![0usize,0,0];

	// Print the inputted data
    println!("The input was:");
    print_indented(file.as_str(), 10);

	// Iterate through the split up file and find each section's sum
    for split in split_file {

    	// Split into a numbers array
		let numbers: Vec<&str> = split.split('\n').collect();
    	let mut sum: usize = 0; // Local sum variable

    	// Go through the numbers, turn the into ints and add them to the sum
		for number in numbers {
		
			if number.is_empty() { continue; } // Ignore empty values
			
			let number_int: usize = number.parse().unwrap();
			sum += number_int;
		}
		
		// Update the max Calories
		if max_cals[0] < sum {
			max_cals[2] = max_cals[1];
			max_cals[1] = max_cals[0];
			max_cals[0] = sum;
		} else if max_cals[1] < sum {
			max_cals[2] = max_cals[1];
			max_cals[1] = sum;
		} else if max_cals[2] < sum {
			max_cals[2] = sum;
		}
    }

	println!("The max Calories held by one elf is: {max_cal}", max_cal = max_cals[0]);
	println!("The top 3 Calories held by elves are {max_cals:?}");
	println!("The total of those top 3 are {total}", total = max_cals[0] + max_cals[1] + max_cals[2]);
}

// Function to indent a text and print it, does it line by line.
fn print_indented(text: &str, indent: usize) {
	for line in text.split('\n') {
		println!("{0:<indent$}{1}", "", line);
	}
}
