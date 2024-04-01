Functions
----------------------

## What are functions?
A language supports functions when it provides syntax to establish a block of code that can be encapsulated into it's own little block

Think of functions like little lego bricks of logic. You can break up logic into little blocks and pieces that can be reused multile times.

When you want to have a process become reusable and isolated, then a function is the best piece of syntax to support that.

For example: 
  - We have a math algorithm that we want to execute multiple times in a codebase
  - We just want to break up our codebase into different logical sections as compared to one long script
  - We have an object that we want changed in the same way each time we interact with it

## Common Function syntax
The most common functions include:
- Explicit Notation - func alias (arguments) response {} 
- Shorthand Notation - alias (arguments) response => {}


## Lesson structure
Each lesson will show the process of how to declare a function
- Information about the most common parts of a function
  - function alias
  - arguments
  - response
  - block of code to execute


## Functions in Depth
Functions are a very old concept in programming. They go as far back as 1947 (see [Wikipedia](https://en.wikipedia.org/wiki/Function_(computer_programming))).

The concept derives from [functions in mathematics](https://en.wikipedia.org/wiki/Function_(mathematics))

They have multiple different names they are known by
- function
- subprogram
- procedure 
- method
- routine
- subroutine 
- callable unit

They consist of four main parts
  - function alias
  - arguments
  - response
  - block of code to execute

### Alias
The function alias is a variable that the function is known by. Similar to variables for primatives, it's the allocated name that programmers can use to interact with the function

### Arguments
The function arguments are unique variables that are specific to the block of code within the function. They cannot be referenced outside of the function. They are initiatlized variables unique to just that particular section of code (called a scope)

### Response
Functions commonly allow for the programmer to return a response to the function. This is not required, but can be utilized to simplify logic. The response can be any data type allowable within the program. Strongly typed languages require the response to be the data type indicated

### Block of Code to execute
The main benefit of a function is that they provide a block of code to execute. This block of code has it's own unique variables that cannot be accessed outside of the function. 

The block of code can be indicated by many things in many languages. Most of the time it's indicated by `{}`

### Executing a function
A function itself is like a lego brick. Just because you design a lego brick, doesn't mean you are using it. To use a function. you do the following:

- define the function and it's alias
- Execute the function (also called `calling` the function)
- Provide your arguments to the function when you execute the function

## In summary
Function syntax was established in order to:
- Support encapsulated blocks of code
- The block of code can be executed from multiple locations
- The block of code can recieve arguments and respond with a value
- The block of code can be reused multiple times
- Syntax and requirements of functions will vary from language to language