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
	// let day: i32;
	// let month: i32;
	// let month_name: char;
	// let year: i32;
	// let second: i32;
	// let minute: i32;
	// let hour: i32;
	// let holiday: bool;
	// let holiday_name: char;
	// let tempurater: f32;


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
	let mut day: i32 = 7;
	let mut month: i32 = 3;
	let mut month_name: String = "March".to_string();
	let mut year: i32 = 1995;
	let second: i32 = 60;
	let minute: i32 = 60;
	let hour: i32 = 10;
	let mut holiday: bool = false;
	let holiday_name: String = "No".to_string();
	let tempurater: f32 = 78.06;

	// Exercise 4 - Initialize 10 variables using the inferred type notation
	let mut day1 = 6;
	let mut month1 = 7;
	let mut month_name1 = "July".to_string();
	let mut year1 = 1995;
	let second1 = 45;
	let minute1 = 25;
	let hour1 = 20;
	let mut holiday1 = false;
	let holiday_name1 = "No".to_string();
	let tempurater1 = 85.64;

	// Exercise 6 - Print out all 20 variables declared above, along with the a guess at what will be printed out on the screen
	// IE. println!("{} = true", secondBool);
	println!("{} {} {} {} {} {} {} {} {} {}", day, month, month_name, year, second, minute, hour, holiday, holiday_name, tempurater);
	println!("{} {} {} {} {} {} {} {} {} {}", day1, month1, month_name1, year1, second1, minute1, hour1, holiday1, holiday_name1, tempurater1);

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
	day = 1;
	month = 8;
	month_name = "August".to_string();
	year = 1999;
	holiday = false;
	// Exercise 8 - Update 5 of the variables you initialized to a new value
	day1 = 17;
	month1 = 2;
	month_name1 = "Febuary".to_string();
	year1 = 2023;
	holiday1 = false;
	// Exercise 9 - Print out the 10 variables you updated
	println!("{} {} {} {} {}", day, month, month_name, year, holiday);
	println!("{} {} {} {} {}", day1, month1, month_name1, year1, holiday1);

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
