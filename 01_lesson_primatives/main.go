package main

import (
	"fmt"
)

func main() {
	// Within a programming language, there are three actions that can be done on a single variable
	// Instantiation, Initialization, and Setting

	// This vocabulary is fairly important for a lot of future concepts, it's highly recommended to understand the difference.

	// Instantiation is the process of creating a brand new variable, but not giving it a value.
	// Below, we instantiate four different variables for the four common primatives

	var myBool bool
	var myFloat float32
	var myInt int32
	var myString string

	// fmt.Println allows us to print out items to our console
	// When we print out the instantiated variables, they result in what Go refers to as empty objects. You should see `false 0 0 ` as the result of the second line
	// That is because those are the default values for go variables
	fmt.Println("1. Instantiated Variables -------------------")
	fmt.Println(myBool, myFloat, myInt, myString)

	// Exercise 1. Instantiate 10 different variables, varying between all of the primative types. Do not provide any of them with values.

	// Exercise 2. Print out your 10 instantiated variables, and also print out what you think the default value will be when you print it.
	// fmt.Println(variableName, defaultValueGuess)
	// For example, if I had a variable named myBool, it would be as such
	// fmt.Println(myBool, false)

	// Initalization is when we provide an `initial` value for an instantiated variable
	// In go, this can be done one of two ways

	// Var notation
	var secondBool bool = true
	var thirdBool = true

	// Colon notation
	fourthBool := true

	// Notice how in var notation, the type of the object is optional, Golang can sense the type instantiated based on the initalized value
	// In colon notation, the type cannot be provided, it only does the process through variable identification

	// Notice how all three values printed out are now `true true true`. As the variables are now instantiated and initalized
	fmt.Println("2. Initalized Variables -------------------")
	fmt.Println(secondBool, thirdBool, fourthBool)

	// Exercise 3. Initialize 10 variables using var notiation with the type declared

	// Exercise 4 - Initialize 10 variables using var notation without the type declared

	// Exercise 5 - Initialize 10 variables using colon notation

	// Exercise 6 - Print out all 30 varibales declared above, alnog with the a guess at what will be printed out on the screen

	// Setting variables is the process of overwriting variable values in order to change them to a new value
	// For example, let's update the value of both some of the initalized and instantiated variables

	myBool = true
	myString = "test"
	secondBool = false
	thirdBool = true

	fmt.Println("3. Set Variables -------------------")
	fmt.Println(myBool, myString, secondBool, thirdBool)

	// By updating variables, we can `set` them to a new value. This allows us to have dynamic systems that adjust and change over time
	// Exercise 7 - Update 5 of the variables you instantiated to a new value

	// Exercise 8 - Update 5 of the variables you initialized to a new value

	// Exercise 9 - Print out the 10 variables you updated

	// In summary
	// Variables can be
	// - instantiated
	// - initalized
	// - set (updated)

	// We've executed those actions on a number of `primative` variables. The same thing could be done with more complex variables as well.
	// Instantiating, initalizing and setting data on variables is a fundemental part of programming languages

}
