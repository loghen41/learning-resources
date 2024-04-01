
// Exercise 1. As a recap to last week, Create a for loop that prints out a number from 30 to 60

console.log("1. Exercise one answer below -------------------");

// Functions
// Functions are encapulated blocks of code that allow you to define an action to be taken and then execute dmultiple times
// For example, the function below has it's own use of the variable `myString`, and they have different values

let myString = "outside of function string";

let myFirstFunction = function(argumentString) {
	let myString = argumentString;
	console.log(myString);
}

console.log(myString);
myFirstFunction("A different string");
myFirstFunction("A third string");

// Functions are normally delcared outside of the `main` function block.
// However, all of the functions we interact with will be declared locally inside of the main function

// To understand this in detail, we are going to break down the function into separate items.

// Exercise 2. Declare three integers, it doesn't matter what the values are, then print them out
// For example

let myInt = 503;
let mySecondInt = 24;
let myThirdInt = 2000;
console.log(`My interger value is: ${myInt} ${mySecondInt} ${myThirdInt}`);

console.log("2. Exercise two values below ----------------");

// How do functions work?
// functions are declared specifically with the syntax func () {}. Everything else can be completely ignored.
// The function won't do anything, but it is possible to build a function that way.
// for example

let mySecondFunction = function() {}

// Exercise 3. Declare a completely empty function
console.log("3. Exercise three values below ----------------");

// Once a function is defined, we need a manner of using the function. We call this executing or calling the function.
// In Go, we call af unction by using it's alias, and then putting the `()` symbol after it. That executes the function
// For example

mySecondFunction(); // The parantehsis calls the function

// Exercise 4. Execute your completely empty function
console.log("4. Exercise four values below ----------------");

// Block of code
// The third part of a function we will deal with is the block of executable code. That's the items inside of the {}
// That block of code can literally do anything, and it will all be unique to inside of the {}
// for example

let myThirdFunction = function() {
	let myInt = 10;                   // Notice how this is the exact same name as the variable above
	let mySecondInt = myInt + 20;     // Each of these variables are unique to just inside of the function
	let myThirdInt = mySecondInt - 5;  // It will have no reference out side of the function whatsoever

	console.log(myThirdInt);
}

// We can call this function multiple times, and it will do the exact same logic again and again
myThirdFunction();
myThirdFunction();
myThirdFunction();

// Exercise 5. Delcare another function, and put whatever something inside of the function block
// Make sure to use at least 3 lines of code, and include one console.log statement
// Execute the function after delcaring it by using the `()` symbol
console.log("5. Exercise five values below ----------------");

// Parameters
// The next part of a function is the ability to provide zero to many parameters to the function
// Parameters in Go are defined in the inital `()` when declaring the function
// These parameters will be placeholders for you to provide values into the block of code for the logic
// They can be any datatype available to the language
// For example

let myFourthFunction = function(myArgument) {
	console.log(`My argument ${myArgument}`);

	let argumentPlusTen = myArgument + 10;
	let finalValue = argumentPlusTen * 2;

	console.log(`My final Value ${finalValue}`);
}
myFourthFunction(myInt);
myFourthFunction(mySecondInt);
myFourthFunction(myThirdInt);

// Notice that depending on the variable I inserted, the println provided a different value.
// This is beacuse I'm inserting the variable as a parameter into the function itself.

// Exercise 5. Delcare another function, provide an argument to the function
// Make sure to use at least 3 lines of code inside of the function, and include one console.log statement
// Execute the function after delcaring it by using the `()` symbol
// Call into your fucntion with your three integer values you declared in Exercise 2
console.log("6. Exercise six values below ----------------")

// Response object
// the last part of the function is the ability to return a response to the caller
// This is useful for creating custom blocks of code that we want to execute the same value on repeatedly
// The response object can be any datatype avaiable in the language
// Javascript doesn't have strict typing, so a function doesn't need to declare a response object. It can just return it
// For example

let myFifthFunction = function(intArgument) {
	let argumentPlusTen = intArgument + 10;
	let finalValue = argumentPlusTen * 2;
	return finalValue
}

let response = myFifthFunction(myInt);
console.log(`My response ${response}`);

// Exercise 7. Create one last function, which takes an integer value, and returns an int
// Call the function with your three variables, and save the value to a variable
// Call console.log() to output your response values, such as shown in the example aboce
console.log("7. Exercise seven values below ----------------");

// In summary
// functions allow for logic to be executed repatedly, common loops are:
// - Support encapsulated blocks of code
// - The block of code can be executed from multiple locations
// - The block of code can recieve arguments and respond with a value
// - The block of code can be reused multiple times
// - Syntax and requirements of functions will vary from language to language
