mod binary_search;

fn main() {
	let xs: [i32; 5] = [1, 2, 3, 4, 5];
	let result = binary_search::index_of(&xs, 4);
	println!("result: {}", result);
}