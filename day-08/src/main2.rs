use std::fs;


pub fn main() {

	// Read the file as a string
	let file: String = fs::read_to_string("input")
		.expect("Should have been able to read the file");

	// The visible grid is the same size as the normal grid except it holds 0s
	// We will update it if a tree is visible
	let (grid, width) = text_to_vec(file);	

	let mut best: usize = 0;
	for x in 0..width {
		for y in 0..grid.len()/width {
			let scenic: usize = calculate_tree_scenic(&grid, width, x, y);
			if scenic > best {best = scenic;}
		}
	}

	println!("{best}");
		
}


fn text_to_vec(text: String) -> (Vec<u8>, usize) {

	// Split out into seperate lines
	let text_by_line: Vec<&str> = text.split('\n').collect();

	
	// Make a blank grid of heights
	let mut grid: Vec<u8> = Vec::new();
	let width : usize = text_by_line[0].len();

	// Append each byte from the linear array, grid[y*width + x] will be used to access
	for line in text_by_line {
		let row : Vec<u8> = line.as_bytes().to_vec();
		for height in row {
			grid.push(height);
		}
	}

	(grid, width)
	
}


fn calculate_tree_scenic(grid: &Vec<u8>, width: usize, target_x: usize, target_y: usize) -> usize {

	let height = grid.len() / width;
	let start_height = grid[target_y*width + target_x];
	let mut total_scenic: usize = 1;

	// 4 directions, for loop to reduce dup code
	for i in 0..4 {

		// (Re)Set the values
		let mut x: usize = target_x;
		let mut y: usize = target_y;
		let mut done: bool = false;
		let mut visible: usize = 0;
		
		while !done {

			// If going to be against a wall then it needs to stop immediatly
			match i {
				0 => done = x >= width - 1	|| done,
				1 => done = x == 0			|| done,
				2 => done = y >= height - 1	|| done,
				3 => done = y == 0			|| done,
				_ => println!("This should not happen")
			}
			if done { break; }

			
			// Increase in the target direction
			match i {
				0 => x += 1,
				1 => x -= 1,
				2 => y += 1,
				3 => y -= 1,
				_ => println!("This should not happen")
			}

			// Needs to stop after the next one if the next tree is too high for the treehouse
			let height = grid[y*width + x];
			done = height >= start_height || done;
			
			//Increase the visible count
			visible += 1;
		}
		
		total_scenic *= visible;
	}

	total_scenic
}
