fn main() {
	// Exercise 1. As a recap to last week, please instantiate and initialize four different variables. (doesn't matter the type)
	// We will be using them in this current lesson

	let my_animal: &str = "cat";

	let is_cat: bool = true;
	let my_color: &str = "black";
	let my_weight: f32 = 15.2;
	let my_height: u32 = 23;

	println!("1. User created variables below -------------------");

	let dog: &str = "dog";
	let lizard: &str = "lizard";
	let horse: &str = "horse";
	let bird: &str = "bird";

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

	println!("2. The above conditional statement will produce the following noise: 'meow'");

	// Exercise 3. Without doing copy and paste, create an `if` statement for your very first variable (without an else). Make the if branch execute
	// For example.
	if dog == "dog" {
		println!("Dogo");
	}

	println!("{}", 4 + 4);
	println!("{}", 3 - 1);
	println!("{}", 5 * 8);
	println!("{}", 3 >= 1);
	println!("{}", 5 <= 1);
	println!("{}", 5 == 5);
	println!("{}", 5 != 3);
	
	let x = 5;

	if 4 == x {
		println!("{}", x);
	} else if x >= 4 {
		println!("That doesn't match fool?");
	} 
	
	if 10 != x {
		println!("No match")
	}

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



	let party = "Awesome";

	if party == "True" {
		println!("Blah");
	}

	if party != "Awesome" {
		println!("Blah Blah")
	} else if party == "Awesome Party!" {
		println!("Oh yeah")
	}

	if party == "" {
		println!("You missed the party");
	} else if party == "Booze" {
		println!("What's a party?");
	} else {
		println!("You found the secret party. Choose your weapon!");
	}


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

	let player = "Wizard";
	
	match player {
		"Warrior" => println!("Very Strong"),
		"Wizard" => println!("FIREBALL!!!"),
		"Troll" => println!("Ugly Fella"),
		"Obor" => println!("All for the club"),
		"Elf" => println!("Pointy Ears"),
		"Dwarf" => println!("Small Fry"),
		_ => println!("You are not part of this party")
	}

	// In summary
	// Conditionals allow for logic to be executed on conditions, common conditionals are:
	// - if, else if, else
	// - switch, case
}