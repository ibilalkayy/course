// Addition function

fn addition(a: u32,  b: u32) {
	println!("Result of addition: {}", a + b);
}

// Subtraction function 
fn subtraction(a: u32, b: u32){
	println!("Resut of Substration: {}", a - b);
}

// Multiplication function (Return the result)
fn multiplication(a: u32, b: u32) -> u32{
	let result: u32 = a * b;
	return result;
}

// Division function (Return the result)

fn division(a: u32, b: u32) -> u32 {
	let result: u32 = a / b;
	return result;
}

// Main function

fn main (){
	addition(10, 5);
	subtraction(50, 20);
	let count = multiplication(5, 10);
	println!("The Result of Multiplication: {}", count);
	let divide = division(100, 5);
	println! ("The Result of Division: {}", divide);

}



