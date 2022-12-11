use std::fs;


pub fn main() {

	// Read the file as a string
	let file: String = fs::read_to_string("input")
		.expect("Should have been able to read the file");

	// The visible grid is the same size as the normal grid except it holds 0s
	// We will update it if a tree is visible
	let (grid, width, mut visible) = text_to_vec(file);	
	

	// Iterate along the grid, testing first going upward then downward
	for x in 0..width {
	
		let mut done: bool = false;
		let mut y: usize = 0;
		let mut max_height: u8 = 0; 
		
		while !done {
			if grid[y*width + x] > max_height {
				visible[y*width + x] += 1;
				max_height = grid[y*width + x];
			}
			
			y += 1;
			
			if max_height == 9 || y >= grid.len() / width {
				done = true;
			}
		}

		done = false;
		y -= 1;
		max_height = 0;

		while !done {
			
			if grid[y*width + x] > max_height {
				visible[y*width + x] += 1;
				max_height = grid[y*width + x];
			}
			
			
			if max_height == 9 || y == 0 {
				done = true;
			}
			else { y -= 1; }	
		}	
	}

	// Iterate up the grid, testing first going left then right
	for y in 0..grid.len()/width {
	
		let mut done: bool = false;
		let mut x: usize = 0;
		let mut max_height: u8 = 0; 
		
		while !done {
			if grid[y*width + x] > max_height {
				visible[y*width + x] += 1;
				max_height = grid[y*width + x];
			}
			
			x += 1;
			
			if max_height == 9 || x >= width {
				done = true;
			}
		}

		done = false;
		x -= 1;
		max_height = 0;

		while !done {
			
			if grid[y*width + x] > max_height {
				visible[y*width + x] += 1;
				max_height = grid[y*width + x];
			}
			
			
			if max_height == 9 || x == 0 {
				done = true;
			}
			else { x -= 1; }	
		}	
	}

	// Count the number of visible trees
	let mut visible_count: usize = 0;
	
	for x in 0..width {
		for y in 0..grid.len()/width {
			if visible[y*width + x] >= 1 {
				visible_count += 1
			}
		}
	}
	

	println!("{:?}", visible_count);
		
}


fn text_to_vec(text: String) -> (Vec<u8>, usize, Vec<u8>) {

	// Split out into seperate lines
	let text_by_line: Vec<&str> = text.split('\n').collect();

	
	// Make a blank grid of heights
	let mut grid: Vec<u8> = Vec::new();
	let mut blank_grid: Vec<u8> = Vec::new();
	let width : usize = text_by_line[0].len();

	// Append each byte from the linear array, grid[y*width + x] will be used to access
	for line in text_by_line {
		let row : Vec<u8> = line.as_bytes().to_vec();
		for height in row {
			grid.push(height);
			blank_grid.push(0);
		}
	}

	(grid, width, blank_grid)
	
}
