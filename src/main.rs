extern crate rand;

mod binary_search;
mod random_sequence;

fn main() {
	let xs: [i32; 5] = [1, 2, 3, 4, 5];
	let result = binary_search::index_of(&xs, 4);
	println!("result: {}", result);

	random_sequence::random_sequence(10, 50.0, 100.0);
	
}