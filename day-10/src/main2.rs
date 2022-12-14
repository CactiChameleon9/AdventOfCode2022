use std::fs;


pub fn main() {

	// Read the file as a string
	let file: String = fs::read_to_string("input")
		.expect("Should have been able to read the file");

	let instructions: Vec<&str> = file.split('\n').collect();

	let mut register_history: Vec<isize> = Vec::new();
	let mut register: isize = 1;

	for instruction in instructions {
		if instruction.is_empty() { continue; } //ignore empty

		// Process the noop command
		if instruction == "noop" {
			register_history.push(register);
		}

		// Process the addx command
		else {
			let amount: isize = instruction.split(' ').collect::<Vec<&str>>()[1].parse().unwrap();
			
			register_history.push(register);
			
			register += amount;
			register_history.push(register);
		}
	}

	let mut crt: String = String::new();

	for cycle in 0..register_history.len() {

		if cycle % 40 == 0  {
			crt += "\n";
		}
		
		let pixel_diff : isize = register_history[cycle] - (cycle % 40)  as isize;
		if pixel_diff == 0 || pixel_diff == 1 || pixel_diff == 2  {
			crt += "#";
		} else {
			crt += ".";
		}

	}
	print!("{crt}");

	
}
