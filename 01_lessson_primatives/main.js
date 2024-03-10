// Within a programming language, there are three actions that can be done on a single variable
// Instantiation, Initialization, and Setting

// This vocabulary is fairly important for a lot of future concepts, it's highly recommended to understand the difference.

// Instantiation is the process of creating a brand new variable, but not giving it a value.
// Below, we instantiate four different variables for the four common primatives

let myBool;
let myFloat;
let myInt;
let myString;

// console.log allows us to print out items to our console
// When we print out the instantiated variables, they result in what Javascript refers to as empty objects. You should see `undefined undefined undefined undefined` as the result of the second line
// That is because undefined is the default value for variables that do not have a `defined value` in Javascirpt
console.log("1. Instantiated Variables -------------------");
console.log(myBool, myFloat, myInt, myString);

// Exercise 1. Initialize 10 different variables, varying between all of the primative types. Do not provide any of them with values.

// Exercise 2. Print out your 10 instantiated variables,

// Initalization is when we provide an `initial` value for an instantiated variable
// In Javascript, this can be done one of three ways

// Var notation
let secondBool = true;

// Let notation
let thirdBool = true;

// const notation
const fourthBool = true;

// The difference between var and let is nuanced at this point in time. It will be explained more in a future lesson. The key takeaway for now is that let is preferred to var.
// Const will not let you adjust the value of the variable. It's a `Constant`. Only use this for values that will not be changed

// Notice how all three values printed out are now `true true true`. As the variables are now instantiated and initalized
console.log("2. Initalized Variables -------------------");
console.log(secondBool, thirdBool, fourthBool);

// Exercise 3. Initialize 10 variables using var notiation 

// Exercise 5 - Initialize 10 variables using let notation

// Exercise 6 - Initialize 10 variables using const notation

// Exercise 6 - Print out all 30 varibales declared above, along with the a guess at what will be printed out on the screen
// For example, console.log(myBool, true)

// Setting variables is the process of overwriting variable values in order to change them to a new value
// For example, let's update the value of both some of the initalized and instantiated variables

myBool = true;
myFloat = 2.0;
myInt = 3;
myString = "test";
secondBool = false;
thirdBool = true;

console.log("3. Set Variables -------------------");
console.log(myBool, myFloat, myInt, myString, secondBool, thirdBool);

// By updating variables, we can `set` them to a new value. This allows us to have dynamic systems that adjust and change over time
// `const` variables cannot be udpated, as they are constants

// Exercise 7 - Update 5 of the variables you instantiated to a new value

// Exercise 8 - Update 5 of the variables you initialized to a new value

// Exercise 9 - Print out the 10 variables you updated

// In Javascript as a non-typed language, you are free to change the values of these prmatives freely.

myBool = 2.0;
myFloat = "test";
myInt = true;
myString = 3;

// However, it is recommended that you keep a variable to a single data type, as it simplifies writing and reading programming languages


// In summary
// Variables can be
// - instantiated
// - initalized
// - set (updated)

// We've executed those actions on a number of `primative` variables. The same thing could be done with more complex variables as well.
// Instantiating, initalizing and setting data on variables is a fundemental part of programming languages


