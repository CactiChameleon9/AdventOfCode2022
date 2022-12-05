use std::fs;

struct Instruction {
	amount : usize,
	from : usize,
	to : usize,
}

pub fn main() {

	// Read the file as a string
	let file: String = fs::read_to_string("input")
		.expect("Should have been able to read the file");

	// Separate the piles and the instructions
	let piles_text = file.split("\n\n").collect::<Vec<&str>>()[0];
	let instructions_text = file.split("\n\n").collect::<Vec<&str>>()[1];

	let mut piles: Vec<Vec<&str>> = process_piles(piles_text);
	let instructions: Vec<Instruction> = process_instructions(instructions_text);

	// Carry out every instruction
	for instruction in instructions{

		// Follow the instruction of moving from one pile to another the correct amount of times
		for _amount in 0..instruction.amount{
			let letter: &str = piles[instruction.from].pop().unwrap();
			piles[instruction.to].push(letter);
		}
	}

	// Print the final top crates (concatenated)
	for pile in piles {
		print!("{}", pile[pile.len() - 1])
	}
	println!(); //remove annoying new-line symbol after concatenation
}


fn process_piles(text: &str) -> Vec<Vec<&str>> {
	let mut piles: Vec<Vec<&str>> = Vec::new();

	// Find out the width and the height
	let text_lines: Vec<&str> = text.split('\n').collect();
	let pile_height: usize = text_lines.len();
	let pile_width: usize = text_lines[0].len(); //assume whitespace-filled

	const SEPARATION_WIDTH: usize = 2; //how spaced out the rows are visually

	// Iterate through the rows, adding the cols to a pile and pushing that pile onto the piles
	let mut row = 0;
	
	while row < pile_width {
		row += SEPARATION_WIDTH; // each row as 2 separations either side 
	
		let mut pile: Vec<&str> = Vec::new();
		
		let mut col = pile_height - 1; //disclude the first identifier row
		while col > 0 {
			col -=1;
		
			let letter: &str = &text_lines[col][row-1..row];
			
			if letter == " " || letter.is_empty() {
				break; //stop once an empty value is reached
			}
			
			pile.push(letter);
		}
		
		piles.push(pile);
		
		row += SEPARATION_WIDTH; // each row as 2 separations either side
	}
	
	piles
}

fn process_instructions(text: &str) -> Vec<Instruction> {

	let mut instructions: Vec<Instruction> = Vec::new();

	for text in text.split('\n').collect::<Vec<&str>>() {
	
		if text.is_empty() { continue; } // Ignore empty

		// Split the instruction into each word
		let words: Vec<&str> = text.split(' ').collect();

		// "move 3 from 1 to 3" is the structure
		// "move" = [0], "3" = [1], etc.
		let amount: usize = words[1].parse().unwrap();
		let from: usize = words[3].parse::<usize>().unwrap() - 1; // make the index start from 0, so -1
		let to: usize = words[5].parse::<usize>().unwrap() - 1;

		// Create the instruction struct
		let instruction: Instruction = Instruction{amount, from, to};

		// Add the instruction to the list
		instructions.push(instruction)
		
	}

	instructions
}
