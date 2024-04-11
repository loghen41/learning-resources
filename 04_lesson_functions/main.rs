fn main() {
	// Exercise 1. As a recap to last week, Create a while loop that prints out a number from 30 to 60

	println!("1. Exercise one answer below -------------------");
	
	let mut number = 1;

	while number < 10 {
		println!("{}", number);

		number = number + 1;
	}

	// Functions
	// Functions are encapulated blocks of code that allow you to define an action to be taken and then execute dmultiple times
	// For example, the function below has it's own use of the variable `myString`, and they have different values

	let my_string: &str = "outside of function string";

	fn my_first_function(argument_string: &str) {
		let my_string: &str = argument_string;
		println!("{}", my_string);
	}

	println!("{}", my_string);
	my_first_function("A different string");
	my_first_function("A third string");

	// Functions are normally delcared outside of the `main` function block.
	// However, all of the functions we interact with will be declared locally inside of the main function

	// To understand this in detail, we are going to break down the function into separate items.

	// Exercise 2. Declare three integers, it doesn't matter what the values are, then print them out
	// For example

	let my_int: i32 = 503;
	let my_second_int: i32 = 24;
	let my_third_int: i32 = 2000;
	println!("My interger value is: {} {} {}", my_int, my_second_int, my_third_int);

	println!("2. Exercise two values below ----------------");

	// How do functions work?
	// functions are declared specifically with the syntax func () {}. Everything else can be completely ignored.
	// The function won't do anything, but it is possible to build a function that way.
	// for example

	fn my_second_function() {

	}

	// Exercise 3. Declare a completely empty function
	println!("3. Exercise three values below ----------------");

	// Once a function is defined, we need a manner of using the function. We call this executing or calling the function.
	// In Go, we call af unction by using it's alias, and then putting the `()` symbol after it. That executes the function
	// For example

	my_second_function(); // The parantehsis calls the function

	// Exercise 4. Execute your completely empty function
	println!("4. Exercise four values below ----------------");

	// Block of code
	// The third part of a function we will deal with is the block of executable code. That's the items inside of the {}
	// That block of code can literally do anything, and it will all be unique to inside of the {}
	// for example

	fn my_third_function() {
		let my_int: i32 = 10;                   // Notice how this is the exact same name as the variable above
		let my_second_int: i32 = my_int + 20;     // Each of these variables are unique to just inside of the function
		let my_third_int: i32 = my_second_int - 5; // It will have no reference out side of the function whatsoever

		println!("{}", my_third_int)
	}

	// We can call this function multiple times, and it will do the exact same logic again and again
	my_third_function();
	my_third_function();
	my_third_function();

	// Exercise 5. Delcare another function, and put whatever something inside of the function block
	// Make sure to use at least 3 lines of code, and include one println! statement
	// Execute the function after delcaring it by using the `()` symbol
	println!("5. Exercise five values below ----------------");

	// Parameters
	// The next part of a function is the ability to provide zero to many parameters to the function
	// Parameters in Go are defined in the inital `()` when declaring the function
	// These parameters will be placeholders for you to provide values into the block of code for the logic
	// They can be any datatype available to the language
	// For example

	fn my_fourth_function(my_argument: i32) {
		println!("My argument {}", my_argument);

		let argument_plus_ten: i32 = my_argument + 10;
		let final_value: i32 = argument_plus_ten * 2;

		println!("My final Value {}", final_value);
	}
	my_fourth_function(my_int);
	my_fourth_function(my_second_int);
	my_fourth_function(my_third_int);

	// Notice that depending on the variable I inserted, the println provided a different value.
	// This is beacuse I'm inserting the variable as a parameter into the function itself.

	// Exercise 5. Delcare another function, provide an argument to the function
	// Make sure to use at least 3 lines of code inside of the function, and include one println! statement
	// Execute the function after delcaring it by using the `()` symbol
	// Call into your fucntion with your three integer values you declared in Exercise 2
	println!("6. Exercise six values below ----------------");

	// Response object
	// the last part of the function is the ability to return a response to the caller
	// This is useful for creating custom blocks of code that we want to execute the same value on repeatedly
	// The response object can be any datatype avaiable in the language
	// The response type is declared by placing a -> character after the inital (), and then the response to return
	// For example

	fn my_fifth_function(int_argument: i32) -> i32 {
		let argument_plus_ten: i32 = int_argument + 10;
		let final_value: i32 = argument_plus_ten * 2;
		return final_value
	}

	let response: i32 = my_fifth_function(my_int);
	println!("My response {}", response);

	// Exercise 7. Create one last function, which takes an integer value, and returns an int
	// Call the function with your three variables, and save the value to a variable
	// Call println!() to output your response values, such as shown in the example above
	println!("7. Exercise seven values below ----------------");

	let my_new_int: i32 = 10;

	fn my_seventh_function(my_new_int: i32) -> i32 {
		let argument_value: i32 = my_new_int * 2;
		let final_value: i32 = argument_value + 5;
		return final_value
	}

	let response: i32 = my_seventh_function(my_new_int);
	println!("New Number {}", response);


	// Build custom function to alter a string

	let my_string: &str = "Ryan";

	fn alter_string(my_string: &str) -> String {
		let result = my_string.replace("Ryan", "Logan");
		return result
	}

	let response: String = alter_string(my_string);
	println!("My name is {}", my_string);
	println!("My New Name is {}", response);

	// In summary
	// functions allow for logic to be executed repatedly, common loops are:
	// - Support encapsulated blocks of code
	// - The block of code can be executed from multiple locations
	// - The block of code can recieve arguments and respond with a value
	// - The block of code can be reused multiple times
	// - Syntax and requirements of functions will vary from language to language
}
