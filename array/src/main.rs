
fn main (){
	// Original array
	let array: [i32; 5] = [10, 20, 30, 40, 50];
	let first_element = array[0];
	let last_element = array[4];

	// sum of first and last element

	let sum = first_element + last_element;
	println!("Sum of first and last element {}", sum);

	// slice of middle elements 20 30 40

	let middle_slice = &array[1..=3];
	println! ("slice of middle {:?}", middle_slice);

}