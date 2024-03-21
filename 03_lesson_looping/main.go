package main

import "fmt"

func main() {
	// Exercise 1. As a recap to last week, please instantiate and initialize a variable. (doesn't matter the type)
	// Then use that variable in an if, else if, else statement

	fmt.Println("1. Exercise one answer below -------------------")

	// For loop
	// Loops build the ability to create a repeating action in code
	// For example, the for loop statement below prints out a statement each 10 times

	for i := 0; i < 10; i++ {
		fmt.Println("my number %v", i)
	}

	// To understand this in detail, we are going to break down the loop into separate stages.

	// Exercise 2. Declare two integers, it doesn't matter what the values are, then print them out
	// For example

	myInt := 503
	MySecondInt := 832
	fmt.Printf("My intergers are: %v %v \n", myInt, MySecondInt)

	fmt.Println("2. Exercise two values below ----------------")

	// How does the for loop work?
	// Our programming language uses a special operator to designate when a statement ends the semicolon ;
	// A for loop is actually just three programming statements all put together on one line
	// statement 1: A variable is declared (usually an integer)
	// Statement 2: A stopping point is delcared (usually a comparison with another integer)
	// Statement 3: An action is delcared that adjusts the variable in step one to make it reach the stopping condition

	// Broken down, the for loop looks like this:

	for myVariable := 503; // The initial variable is declared
	myVariable < 832;      // A stopping condition is set
	myVariable++ {         // An action is done to make the variable reach the stopping condition
		// The block of code
		fmt.Printf("myNumber %v \n", myVariable)
	}

	// Using our variables from above, it looks like this:
	for i := myInt; i < MySecondInt; i++ {
		fmt.Printf("my Int: %v \n", i)
	}

	// Exercise 3. Write a for loop similar to what was defined above, but using your variables instead
	fmt.Println("3. Exercise three statements below ----------------")

	// Exercise 4. Loops with Conditionals
	// Write a loop that iterates through a series of numbers
	// If the number is even, print out "Even"
	// if the number is odd, print out "Odd"

	// For In loop
	// A for ... in loop allows you to loop over the contents of a larger structure. Such as an array.
	// The golang version of for ... in is by using a for, range
	// For example:

	myFruit := []string{"Apple", "Banana", "Pear", "Mango"}

	// The first argument is the index in the array that you are looping through
	// The second argument is the value from the array that you are at.
	for index, fruit := range myFruit {
		fmt.Printf("index: %v, fruit: %v\n", index, fruit)
	}

	// Exercise 5. Write an array and then loop through the array using the `for range` syntax

	fmt.Println("5. Exercise four statements below ----------------")

	// In summary
	// Looping allow for logic to be executed repatedly, common loops are:
	// - for
	// - for ... in
}
