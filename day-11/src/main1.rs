use std::fs;

const ROUNDS: usize = 20;


struct Monkey {
	id: usize,
	items: Vec<usize>,
	operation: String,
	test_divider: usize,
	true_monkey: usize,
	false_monkey: usize,
	inspections: usize
}

impl Monkey {

	pub fn new(id: usize, items: Vec<usize>, operation: String,
				test_divider: usize, true_monkey: usize,
				false_monkey: usize, inspections: usize) -> Monkey {

		Monkey {	
			id,
			items,
			operation,
			test_divider,
			true_monkey,
			false_monkey,
			inspections
		}
	}
	
	pub fn from_str(id: usize, text: &str) -> Monkey {
	/*
		Monkey 2:
		  Starting items: 79, 60, 97
		  Operation: new = old * old
		  Test: divisible by 13
		    If true: throw to monkey 1
		    If false: throw to monkey 3
	*/

		// Seperate out the text into lines
		let lines: Vec<&str> = text.split('\n').collect();

		// Get the items in string form
		let items_string: Vec<&str> = lines[1].split(": ").collect::<Vec<&str>>()[1].split(", ").collect();

		// Generate the usize list of items, pushing them to the items vec
		let mut items: Vec<usize> = Vec::new();
		for item in items_string {
			items.push(item.parse().unwrap());
		}

		// Get the operation text in string form: "var *+/ number||var"
		let operation: String = lines[2].split(" = ").collect::<Vec<&str>>()[1].to_string();

		// Get the divider, true and false from the text. Trim is used to remove whitespaces (problems ' ')
		let test_divider: usize = lines[3].trim().split(' ').collect::<Vec<&str>>()[3].parse().unwrap();
		let true_monkey: usize = lines[4].trim().split(' ').collect::<Vec<&str>>()[5].parse().unwrap();
		let false_monkey: usize = lines[5].trim().split(' ').collect::<Vec<&str>>()[5].parse().unwrap();

		// Return the completed Monkey
		Monkey::new(
			id,
			items,
			operation,
			test_divider,
			true_monkey,
			false_monkey,
			0
		)
	}

	pub fn has_items(&self) -> bool {
		self.items.is_empty()
	}
	
	pub fn add_item(&mut self, item: usize) {
		self.items.push(item);
	} 

	pub fn run_turn(&mut self) -> (usize, usize) {

		// Retrieve the results from the inspection
		let (inspect_result, worry) : (bool, usize) = self.run_inspection();

		// Set the destination to either the true or false monkey
		let destination: usize;
		if inspect_result {
			destination = self.true_monkey;
		} else {
			destination = self.false_monkey;
		}

		(destination, worry)
	}

	fn run_inspection(&mut self) -> (bool, usize) {

		// Increase the inspections
		self.inspections += 1;

		// Get and remove the old worry value
		let old_worry: usize = self.items[0];
		self.items.remove(0);

		// Split the operation up into its pieces
		let operation_split: Vec<&str> = self.operation.split(' ').collect();
		let operand1: &str = operation_split[0];
		let operator: &str = operation_split[1];
		let operand2: &str = operation_split[2];

		// Assign the correct values to the operands (the old value or the specified value)
		let operand1_usize: usize;
		let operand2_usize: usize;
		
		if operand1 == "old" { operand1_usize = old_worry; }
		else { operand1_usize = operand1.parse().unwrap(); }

		if operand2 == "old" { operand2_usize = old_worry; }
		else { operand2_usize = operand2.parse().unwrap(); }

		// Find the answer to the operation
		let mut worry: usize;
		match operator {
			"+" => worry = operand1_usize + operand2_usize,
			"-" => worry = operand1_usize - operand2_usize,
			"*" => worry = operand1_usize * operand2_usize,
			"/" => worry = operand1_usize / operand2_usize,
			_ => worry = 0
		}

		// Divide worry by 3, rounding down
		worry -= worry % 3;
		worry /= 3;

		// Return if the number is divisible by the test_divider
		(worry % self.test_divider == 0, worry)
		
	}
}



pub fn main() {

	// Read the file as a string
	let file: String = fs::read_to_string("input")
		.expect("Should have been able to read the file");

	let monkey_strings: Vec<&str> = file.split("\n\n").collect();

	let mut monkeys: Vec<Monkey> = Vec::new();

	for id in 0..monkey_strings.len() {
		if monkey_strings[id].is_empty() { continue; } //ignore empty
	
		let m: Monkey = Monkey::from_str(id, monkey_strings[id]);
		monkeys.push(m);
	}

	// Run 20 rounds
	for _rounds in 0..ROUNDS {
		// Each round goes through each monkey in turn
		for id in 0..monkeys.len() {
			// Each monkey needs to run until their list is empty
			while !monkeys[id].has_items() {
				let (destination, worry): (usize, usize) = monkeys[id].run_turn();
				monkeys[destination].add_item(worry);
			}
		}
	}

	let mut highest: [usize; 2] = [0, 0];

	// Print the inspections in a nice format and update the highest
	for monkey in monkeys {
		println!("Monkey {} inspected items {} times", monkey.id, monkey.inspections);

		if monkey.inspections > highest[0] {
			highest[1] = highest[0];
			highest[0] = monkey.inspections;
		}
		else if monkey.inspections > highest[1] {
			highest[1] = monkey.inspections;
		}
	}

	println!("The monkey business is {}", highest[0] * highest[1]);
}
