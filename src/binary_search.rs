//! Binary search an array of sorted integers

pub fn index_of(arr: &[i32], key: i32) -> usize {
	let mid: usize = arr.len() / 2;
	if key < arr[mid] {
		index_of(&arr[0 .. mid], key)
	} else if key > arr[mid]{
		index_of(&arr[mid + 1 .. arr.len()], key) + mid + 1
	} else {
	    mid
	}
}

#[test]
fn it_works() {
	let xs: [i32; 5] = [1, 2, 3, 4, 5];
	assert_eq!(index_of(&xs, 1), 0);
	assert_eq!(index_of(&xs, 2), 1);
	assert_eq!(index_of(&xs, 3), 2);
	assert_eq!(index_of(&xs, 4), 3);
	assert_eq!(index_of(&xs, 5), 4);
}
