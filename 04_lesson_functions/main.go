package main

import "fmt"

func main() {
	// Exercise 1. As a recap to last week, Create a for loop that prints out a number from 30 to 60

	fmt.Println("1. Exercise one answer below -------------------")

	// Functions
	// Functions are encapulated blocks of code that allow you to define an action to be taken and then execute dmultiple times
	// For example, the function below has it's own use of the variable `myString`, and they have different values

	myString := "outside of function string"

	myFirstFunction := func(argumentString string) {
		myString := argumentString
		fmt.Println(myString)
	}

	fmt.Println(myString)
	myFirstFunction("A different string")
	myFirstFunction("A third string")

	// Functions are normally delcared outside of the `main` function block.
	// However, all of the functions we interact with will be declared locally inside of the main function

	// To understand this in detail, we are going to break down the function into separate items.

	// Exercise 2. Declare three integers, it doesn't matter what the values are, then print them out
	// For example

	myInt := 503
	mySecondInt := 24
	myThirdInt := 2000
	fmt.Printf("My interger value is: %v %v %v \n", myInt, mySecondInt, myThirdInt)

	fmt.Println("2. Exercise two values below ----------------")

	// How do functions work?
	// functions are declared specifically with the syntax func () {}. Everything else can be completely ignored.
	// The function won't do anything, but it is possible to build a function that way.
	// for example

	mySecondFunction := func() {}

	// Exercise 3. Declare a completely empty function
	fmt.Println("3. Exercise three values below ----------------")

	// Once a function is defined, we need a manner of using the function. We call this executing or calling the function.
	// In Go, we call af unction by using it's alias, and then putting the `()` symbol after it. That executes the function
	// For example

	mySecondFunction() // The parantehsis calls the function

	// Exercise 4. Execute your completely empty function
	fmt.Println("4. Exercise four values below ----------------")

	// Block of code
	// The third part of a function we will deal with is the block of executable code. That's the items inside of the {}
	// That block of code can literally do anything, and it will all be unique to inside of the {}
	// for example

	myThirdFunction := func() {
		myInt := 10                   // Notice how this is the exact same name as the variable above
		mySecondInt := myInt + 20     // Each of these variables are unique to just inside of the function
		myThirdInt := mySecondInt - 5 // It will have no reference out side of the function whatsoever

		fmt.Println(myThirdInt)
	}

	// We can call this function multiple times, and it will do the exact same logic again and again
	myThirdFunction()
	myThirdFunction()
	myThirdFunction()

	// Exercise 5. Delcare another function, and put whatever something inside of the function block
	// Make sure to use at least 3 lines of code, and include one fmt.Println statement
	// Execute the function after delcaring it by using the `()` symbol
	fmt.Println("5. Exercise five values below ----------------")

	// Parameters
	// The next part of a function is the ability to provide zero to many parameters to the function
	// Parameters in Go are defined in the inital `()` when declaring the function
	// These parameters will be placeholders for you to provide values into the block of code for the logic
	// They can be any datatype available to the language
	// For example

	myFourthFunction := func(myArgument int) {
		fmt.Printf("My argument %v \n", myArgument)

		argumentPlusTen := myArgument + 10
		finalValue := argumentPlusTen * 2

		fmt.Printf("My final Value %v \n", finalValue)
	}
	myFourthFunction(myInt)
	myFourthFunction(mySecondInt)
	myFourthFunction(myThirdInt)

	// Notice that depending on the variable I inserted, the println provided a different value.
	// This is beacuse I'm inserting the variable as a parameter into the function itself.

	// Exercise 5. Delcare another function, provide an argument to the function
	// Make sure to use at least 3 lines of code inside of the function, and include one fmt.Println statement
	// Execute the function after delcaring it by using the `()` symbol
	// Call into your fucntion with your three integer values you declared in Exercise 2
	fmt.Println("6. Exercise six values below ----------------")

	// Response object
	// the last part of the function is the ability to return a response to the caller
	// This is useful for creating custom blocks of code that we want to execute the same value on repeatedly
	// The response object can be any datatype avaiable in the language
	// The response type is declared after the initial `()` when declaring the function
	// For example

	myFifthFunction := func(intArgument int) int {
		argumentPlusTen := intArgument + 10
		finalValue := argumentPlusTen * 2
		return finalValue
	}

	response := myFifthFunction(myInt)
	fmt.Printf("My response %v \n", response)

	// Exercise 7. Create one last function, which takes an integer value, and returns an int
	// Call the function with your three variables, and save the value to a variable
	// Call fmt.Printf() to output your response values, such as shown in the example aboce
	fmt.Println("7. Exercise seven values below ----------------")

	// In summary
	// functions allow for logic to be executed repatedly, common loops are:
	// - Support encapsulated blocks of code
	// - The block of code can be executed from multiple locations
	// - The block of code can recieve arguments and respond with a value
	// - The block of code can be reused multiple times
	// - Syntax and requirements of functions will vary from language to language
}
