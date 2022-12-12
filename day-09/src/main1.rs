use std::fs;
use std::collections::HashMap;

pub fn main() {

	// Read the file as a string
	let file: String = fs::read_to_string("input")
		.expect("Should have been able to read the file");

	let movements: Vec<&str> = file.split('\n').collect();

	/* Rope rules:
		Tail-head -> LL,RR,UU,DD = Tail += L,R,U,D
		Tail-head -> LUU,LLU,LDD,LLD,RUU,RRU,RDD,RRD = Tail += LU,LU,LD,LD,RU,RU,RD,RD
		In words:	if on the same x/y axis move toward
					if on diffeerent x and y axis, move diagional

	*/

	let mut head: [isize; 2] = [0, 0];
	let mut tail: [isize; 2] = [0, 0];

	let mut visited: HashMap<[isize; 2], usize> = HashMap::new();	
	visited.insert(tail, 1); // visit 0,0

	for movement in movements {
		if movement.is_empty() { continue; } //skip empty
		
		let direction: [isize; 2] = process_direction(movement.split(' ').collect::<Vec<&str>>()[0]);
		let amount: usize = movement.split(' ').collect::<Vec<&str>>()[1].parse().unwrap();

		for _i in 0..amount {
		
			// Move the head
			head = vec_add(head, direction);

			let difference = vec_rem(tail, head);

			// Move the tail if the distance is greater than 1 by 1 (root 2)
			if vec_len(difference) > 1.5 {
				tail = vec_add(tail, vec_trim(difference));

			// Set the vec to visited
			visited.insert(tail, 1);
			
			}
		}
	}

	let mut visited_total: usize = 0;
	for (_key, _value) in visited {
		if _value == 1 {
			visited_total += 1;
		}
	}

	println!("The number of spaces visited by the tail is {visited_total}")	
	
}

fn process_direction(direction: &str) -> [isize; 2] {
	match direction {
		"U" => [0, 1],
		"D" => [0, -1],
		"R" => [1, 0],
		"L" => [-1, 0],
		_ => [0, 0]
	}
}


fn vec_add(vec1: [isize; 2], vec2: [isize; 2]) -> [isize; 2] {
	[vec1[0] + vec2[0], vec1[1] + vec2[1]]
}

fn vec_rem(vec1: [isize; 2], vec2: [isize; 2]) -> [isize; 2] {
	[-vec1[0] + vec2[0], -vec1[1] + vec2[1]]
}

fn vec_len(vec: [isize; 2]) -> f64 {
	f64::sqrt((isize::pow(vec[0], 2) + isize::pow(vec[1], 2)) as f64)
}

// Trims a vector to a max length 1 on both x and y
fn vec_trim(vec: [isize; 2]) -> [isize; 2] {
	let mut new_vec = [vec[0], vec[1]];
	if new_vec[0] != 0 { new_vec[0] = vec[0] / vec[0].abs(); }
	if new_vec[1] != 0 { new_vec[1] = vec[1] / vec[1].abs() }
	new_vec
}
