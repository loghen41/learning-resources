fn main() {
	// Exercise 1. As a recap to last week, please instantiate and initialize a variable. (doesn't matter the type)
	// Then use that variable in an if, else if, else statement

	println!("1. Exercise one answer below -------------------");

	// While loop
	// Loops build the ability to create a repeating action in code
	// For example, the for loop statement below prints out a statement each 10 times
	
	let mut i = 0;

	while i < 10 {
		println!("my number {}", i);
		i = i + 1;
	}

	// To understand this in detail, we are going to break down the loop into separate stages.

	// Exercise 2. Declare two integers, it doesn't matter what the values are, then print them out
	// For example

	let mut my_int: u32 = 503;
	let my_second_int: u32 = 832;

	println!("My intergers are: {} {} \n", my_int, my_second_int);

	println!("2. Exercise two values below ----------------");

	// How does the while loop work?
	// The while loop is similar to an `if` statement.
	// it expects a boolean value after the statement
	// If you were to put `while true { println!("hello")}` it would literally loop forever
	// We provide the while loop a boolean value, and then do actions in the loop to help us exit the loop

	// Using our variables from above, it looks like this:
	while my_int < my_second_int { // Declare the boolean expression

		// Execute some sort of logic
		println!("my int: {}", my_int);
		
		// Execute logic to make it so our condition eventually ends
		my_int = my_int + 1;
	}

	// Exercise 3. Write a while loop similar to what was defined above, but using your variables instead
	println!("3. Exercise three statements below ----------------");

	// Exercise 4. Loops with Conditionals
	// Write a loop that iterates through a series of numbers
	// If the number is even, print out "Even"
	// if the number is odd, print out "Odd"

	
	// For In loop
	// A for ... in loop allows you to loop over the contents of a larger structure. Such as an array.
	// For example:

	let my_fruit = &["apples", "banana", "pear", "mango"];

	// This allows you to loop through the items in the array and do actions with them
	for fruit in my_fruit {
		println!("I like {}.", fruit);
	}

	// Exercise 5. Write a for ... in loop using an array and then print out statements from the array
	println!("5. Exericse four statements below ------------");

	// In summary
	// Looping allow for logic to be executed repatedly, common loops are:
	// - for
	// - for ... in
}
