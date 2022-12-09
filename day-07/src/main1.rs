use std::fs;
use std::collections::HashMap;


pub fn main() {

	// Read the file as a string
	let file: String = fs::read_to_string("input")
		.expect("Should have been able to read the file");

	let lines: Vec<&str> = file.split('\n').collect();

	let mut current_folder: Vec<String> = Vec::new();
	let mut folder_totals: HashMap<String,usize> = HashMap::new();

	for line in lines {

		if line.is_empty() { continue; } //ignore empty

		let line_split: Vec<&str> = line.split(' ').collect();

		// Update the current folder list
		if line_split[1] == "cd" && line_split[0] == "$" {
		
			if line_split[2] == ".." {
				current_folder.pop(); //go back a dir
				
			} else {

				// Foldername needs to equal the full directory because of duplicate folder names
				let folder_name;
				if !current_folder.is_empty() { 
					folder_name = format!("{}{}", current_folder[current_folder.len() - 1], line_split[2]);
				} else {
					folder_name = line_split[2].to_string();
				}
				
				current_folder.push(folder_name);
			}
		
		}

		// Increase all of the folder totals for the directories we are currently in
		if line_split[0] != "$" && line_split[0] != "dir" {

			let file_size: usize = line_split[0].parse().unwrap();
		
			for folder_name in &current_folder {

				// Get the current folder total
				let folder_total: &usize; //could be empty
				if folder_totals.contains_key(folder_name) {
					folder_total = folder_totals.get(folder_name).unwrap();
				} else { folder_total = &0; }

				// Make a new var for the new total
				let new_folder_total: usize = folder_total + file_size;

				// Update the has table
				folder_totals.insert(folder_name.to_string(), new_folder_total);
			}
		}
	}

	// Find the sum that is < 10000
	let mut sum: usize = 0;

	for (_name, size) in folder_totals {
		if size <= 100000 {
			sum += size
		}
	}

	println!("The total sum of folder less than 100000 is {}", sum);
}

