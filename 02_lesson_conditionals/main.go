package main

import "fmt"

func main() {
	// Exercise 1. As a recap to last week, please instantiate and initialize four different variables. (doesn't matter the type)
	// We will be using them in this current lesson

	myAnimal := "cat"

	isCat := true
	myColor := "black"
	myHeight := 23
	myWeight := 15.2

	fmt.Println("1. User created variables below -------------------")

	// if, else if, else
	// Conditionals are the process of breaking logic into separate branches of code.
	// For example, the switch statement below checks what type of animal is being stored as a variable, and outputs different noises accordingly.

	if myAnimal == "dog" { // If executes an initial check to see if the value matches up
		fmt.Println("bark")
	} else if myAnimal == "lion" { // Else if only excutes if the prior steps are not executed.
		fmt.Println("roar")
	} else if myAnimal == "cat" { // This will continue until a branch is found that resolves to `true`
		fmt.Println("meow")
	} else { // if none of the branches execute to true, and the conditional provides an `else` statement, this is the default action that will occur
		fmt.Println("unknown animal")
	}

	// Exercise 2. fill in what animal noise you think will be produced by the conditional statement above.

	fmt.Println("2. The above conditional statement will produce the following noise: ''")

	// Exercise 3. Without doing copy and paste, create an `if` statement for your very first variable (without an else). Make the if branch execute
	// For example.

	fmt.Println("3. If statement exercises -------------------")

	if isCat == true {
		fmt.Println("It's a cat")
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

	if isCat != true {
		fmt.Println("it's not a cat")
	}

	// Exercise 4. Without doing a copy and paste, create a new `if, else if` statement for your second variable. Have it resolve true in the very first branch
	// for example
	fmt.Println("4. If, Else if statement exercises -------------------")

	if myColor == "black" {
		fmt.Println("It's a black cat")
	} else if myColor == "tabby" {
		fmt.Println("It's a tabby cat")
	}

	// Exercise 5. Without doing a copy and paste, create a new `if, else if` statement for your second variable. Have it resolve true in the second branch
	// for example
	if myColor == "tabby" {
		fmt.Println("It's a tabby cat")
	} else if myColor == "black" {
		fmt.Println("It's a black cat")
	}

	// Exercise 6. Without doing a copy and paste, create a new `if, else if` statement for your second variable. Provide it with four branches. Have not not resolve with any of them.
	// for example
	if myColor == "brown" {
		fmt.Println("It's a brown cat")
	} else if myColor == "white" {
		fmt.Println("It's a white cat")
	} else if myColor == "tuxedo" {
		fmt.Println("It's a tuxedo cat")
	} else if myColor == "orange" {
		fmt.Println("It's a orange cat")
	}

	// Exercise 7. Without doing a copy and paste, create a new `if, else if, else` statement for your third variable. Have it execute on the else portion of your condition
	// for example
	fmt.Println("7. If, Else if, Else statement exercises -------------------")

	if myHeight < 10 {
		fmt.Println("it's a shorty animal")
	} else if myHeight < 20 {
		fmt.Println("it's a short animal")
	} else {
		fmt.Println("it's a normal sized animal")
	}

	// Case, switch, default
	// Case, Switch allows you to take the logic of a longer if, else if, else and condense it into a simpler syntax. This breaks down into switch, case, and default

	switch myAnimal { // Switch is the start of the statement, you declare `switch` and then follow it up with a variable or value that you want to compare
	case "dog": // Case statements indicate different options that the value can resolve to
		fmt.Println("bark")
	case "lion": // Each case statement will only resolve if it matches up with the exact value
		fmt.Println("roar")
	case "cat": // Case statements cannot be repeated, and msut each be unique values
		fmt.Println("meow")
	default: // A default value works similar to an else above, it's the default value if none of the case statements resolve
		fmt.Println("unknown animal")
	}

	// Exercise 8. With your fourth variable, create a switch, case, default statement that resolves to one of the case statements
	// for example
	fmt.Println("8. Switch, case statement exercises -------------------")

	switch myWeight {
	case 10:
		fmt.Println("my weight is 10")
	case 11:
		fmt.Println("my weight is 11")
	case 15.2:
		fmt.Println("my weight is 15.2")
	default:
		fmt.Println("unknown weight")
	}

	// Exercise 9. with your fourth variable, create a brand new switch, case, default statement the doesn't match any of the cases. Have it move to the default case:
	// For example
	switch myWeight {
	case 100:
		fmt.Println("This is a heavy animal")
	case 10:
		fmt.Println("This is a light animal")
	default:
		fmt.Println("This is a normal animal")
	}

	// In summary
	// Conditionals allow for logic to be executed on conditions, common conditionals are:
	// - if, else if, else
	// - switch, case
}
