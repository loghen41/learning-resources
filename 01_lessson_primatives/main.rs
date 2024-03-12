fn main() {
	// Within a programming language, there are three actions that can be done on a single variable
	// Instantiation, Initialization, and Setting

	// This vocabulary is fairly important for a lot of future concepts, it's highly recommended to understand the difference.

	// Instantiation is the process of creating a brand new variable, but not giving it a value.
	// Below, we instantiate four different variables for the four common primatives

	println!("Hello, world!");

	let my_bool: bool;
	let my_float: f32;
	let my_int: u32;
	let my_char: char;

	// println! allows us to print out items to our console
	// Rust does not allow us to do a println for instantiated variables that have not been initialized. 
	// If you uncomment line 21 and try to run the code, it will throw a compiler error stating that the variables have not been initalized
	println!("1. Instantiated Variables -------------------");
	// println!("{} {} {} {}" ,myBool, myFloat, myInt, myString);

	// Exercise 1. Instantiate 10 different variables, varying between all of the primative types. Do not provide any of them with values.

	// Exercise 2. Attempt to print out your 10 instantiated variables
	// See the compiler error provided, and then remove the 10 instantiated variables

	// Initalization is when we provide an `initial` value for an instantiated variable
	// In Rust, this can be done one of two ways

	// Explicit type notation
	let mut second_bool: bool = true;

	// Inferred type notation
	let mut third_bool = true;

	// The type of the object is optional, Rust can sense the type instantiated based on the initalized value

	// Notice how the initalized values can be printed out without a compiler error. As the variables are now instantiated and initalized
	println!("2. Initalized Variables -------------------");
	println!("{} {}", second_bool, third_bool);

	// Exercise 3. Initialize 10 variables using the explicit type notation

	// Exercise 4 - Initialize 10 variables using the inferred type notation

	// Exercise 6 - Print out all 20 variables declared above, along with the a guess at what will be printed out on the screen
	// IE. println!("{} = true", secondBool);

	// Setting variables is the process of overwriting variable values in order to change them to a new value
	// For example, let's update the value of both some of the initalized and instantiated variables

	my_bool = true;
	my_float = 2.0;
	my_int = 3;
	my_char = 'T';
	second_bool = false;
	third_bool = true;

	println!("3. Set Variables -------------------");
	println!("{} {} {} {} {} {}", my_bool, my_float, my_int, my_char, second_bool, third_bool);

	// By updating variables, we can `set` them to a new value. This allows us to have dynamic systems that adjust and change over time
	// Exercise 7 - Update 5 of the variables you instantiated to a new value

	// Exercise 8 - Update 5 of the variables you initialized to a new value

	// Exercise 9 - Print out the 10 variables you updated

	// In Rust, certain variables cannot be mutated, aka updated.
	// These variables are:
	// - `const` variables 
	// - variables not annotated with `mut`

	let cannot_update_this_variable = true;
	const CANNOT_UPDATE_THIS_CONST: bool = true;

	// Exercise 10. Uncomment the following two lines and attempt to compile the code. It will throw an error because these variables cannot be updated.

	// cannot_update_this_variable = false;
	// CANNOT_UPDATE_THIS_CONST = false;

	println!("4. Constant Variables -------------------");
	println!("{} {}", cannot_update_this_variable, CANNOT_UPDATE_THIS_CONST);

	// In summary
	// Variables can be
	// - instantiated
	// - initalized
	// - set (updated)

	// We've executed those actions on a number of `primative` variables. The same thing could be done with more complex variables as well.
	// Instantiating, initalizing and setting data on variables is a fundemental part of programming languages
}
