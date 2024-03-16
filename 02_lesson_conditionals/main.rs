fn main() {
	// Exercise 1. As a recap to last week, please instantiate and initialize four different variables. (doesn't matter the type)
	// We will be using them in this current lesson

	let my_animal: &str = "cat";

	let is_cat: bool = true;
	let my_color: &str = "black";
	let my_weight: f32 = 15.2;
	let my_height: u32 = 23;

	println!("1. User created variables below -------------------");

	// if, else if, else
	// Conditionals are the process of breaking logic into separate branches of code.
	// For example, the switch statement below checks what type of animal is being stored as a variable, and outputs different noises accordingly.

	if my_animal == "dog" { // If executes an initial check to see if the value matches up
		println!("bark");
	} else if my_animal == "lion" { // Else if only excutes if the prior steps are not executed.
		println!("roar");
	} else if my_animal == "cat" { // This will continue until a branch is found that resolves to `true`
		println!("meow");
	} else { // if none of the branches execute to true, and the conditional provides an `else` statement, this is the default action that will occur
		println!("unknown animal");
	}

	// Exercise 2. fill in what animal noise you think will be produced by the conditional statement above.

	println!("2. The above conditional statement will produce the following noise: ''");

	// Exercise 3. Without doing copy and paste, create an `if` statement for your very first variable (without an else). Make the if branch execute
	// For example.

	println!("3. If statement exercises -------------------");

	if is_cat == true {
		println!("It's a cat");
	}

	// Exercise 3. Without doing copy and paste, create another `if` statement for your very first variable. Make sure the if branch DOES NOT execute
	// for example

	// The Allowable operators in an if statement are as follows:
	// == - Equals
	// != - Not Equals
	// <  - Less than
	// <= - Less than Or Equal to
	// >  - Greater than
	// >= - Greater than or Equal to

	// Use these operators in different parts of the exercises below
	
	if is_cat != true {
		println!("it's not a cat");
	}

	// Exercise 4. Without doing a copy and paste, create a new `if, else if` statement for your second variable. Have it resolve true in the very first branch
	// for example
	println!("4. If, Else if statement exercises -------------------");

	if my_color == "black" {
		println!("It's a black cat");
	} else if my_color == "tabby" {
		println!("It's a tabby cat");
	}

	// Exercise 5. Without doing a copy and paste, create a new `if, else if` statement for your second variable. Have it resolve true in the second branch
	// for example
	if my_color == "tabby" {
		println!("It's a tabby cat");
	} else if my_color == "black" {
		println!("It's a black cat");
	}

	// Exercise 6. Without doing a copy and paste, create a new `if, else if` statement for your second variable. Provide it with four branches. Have not not resolve with any of them.
	// for example
	if my_color == "brown" {
		println!("It's a brown cat");
	} else if my_color == "white" {
		println!("It's a white cat");
	} else if my_color == "tuxedo" {
		println!("It's a tuxedo cat");
	} else if my_color == "orange" {
		println!("It's a orange cat");
	}

	// Exercise 7. Without doing a copy and paste, create a new `if, else if, else` statement for your third variable. Have it execute on the else portion of your condition
	// for example
	println!("7. If, Else if, Else statement exercises -------------------");

	if my_weight < 10.0 {
		println!("it's a very light animal");
	} else if my_weight < 15.0 {
		println!("it's a light animal");
	} else {
		println!("it's a normal sized animal");
	}

	// Match statement
	// Match allows you to take the logic of a longer if, else if, else and condense it into a simpler syntax. This breaks down into Match and values

	match my_animal { // match is the start of the statement, you declare `match` and then follow it up with a variable or value that you want to compare
		"dog" => println!("bark"),       // This indicates an option, with an action to do if it resolves correctly
		"lion" => println!("roar"),      // Multiple options can be provided
		"cat" => println!("meow"),       // Each option is evaluated until one resolves to match the value of the variable
		_ => println!("unknown animal"), // If none of the options resolve correctly, a _ operator can be provided which acts as the default option
	}

	// Exercise 8. With your fourth variable, create a Match statement that resolves to one of the options
	// for example
	println!("8. Match exercises -------------------");

	match my_height {
		10 => println!("my height is 10"),
		11 => println!("my height is 11"),
		23 => println!("my height is 23"),
		_ => println!("unknown height"),
	}

	// Exercise 9. with your fourth variable, create a brand new match statement the doesn't match any of the options. Have it move to the default option (ie _):
	// For example
	match my_height {
		100 => println!("This is a tall animal"),
		10 => println!("This is a short animal"),
	 	_ => println!("This is a normal animal"),
	}

	// In summary
	// Conditionals allow for logic to be executed on conditions, common conditionals are:
	// - if, else if, else
	// - switch, case
}
