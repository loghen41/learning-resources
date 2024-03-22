Primatives
----------------------

## What are primatives?
Primative data types are the basic data types that any language sets up.
each language sets up different primitive data types
See Wikipedia entry for the different primatives in each language: https://en.wikipedia.org/wiki/Primitive_data_type

## Common Primatives
The most common primatives are as follows:
- Number                                 0, 1, 2, 3
- String or Char                         "Hello", 'h'
- Boolean                                true, false
- Floating Point numbers (aka Float)     1.1, 2.2, 2.1 (any number with a decimal)   

## Lesson structure
Each lesson will show the process of
- Information about the four primative types in each language 
- Instantiating variables
- Initalizing variables
- Setting variables


## Primatives in Depth

### History of Computing
When computers were first being developed in the early 60-80's. they were largely focused on computational processes.
In fact, recommended viewing would be the Movie "Hidden Figures". Which identifies early history of computing logic at NASA
A "Computer" referred to a human who would "compute" mathematic equations for high difficulty situations

As such, almost all of the terminology, processes, and concepts for "Computer Science", are going to be rooted in "Mathematical Science".

When Hardware computers were developed, the engineers needed a way to input the mathematic data into the system. As it was focused on mathematics, the core data structures were:
- Numbers
- Floating Numbers
- Actions that could be taken to utilize those numbers (We refer to these today as functions)

Sidebar - A function (as defined in a later course), is actually a [mathematics term](https://en.wikipedia.org/wiki/Function_(mathematics)). It is defined as a process of how to take variables and equations and come up with a mathematical answer.

### Why do programming languages have primative types?
Computer engineers were challenged with the process of building a system to allow users to provide basic numbers and use them in equations and get a result.
Computer systems operate off of binary signals (0's and 1's). Engineers needed a system to store basic data as binary that could be accessed by users.

A primative type was developed where the engineer could make a `user facing` type that reflected a certain amount of `binary data`. 

#### Boolean as Binary
A Boolean type (true/false) can be represented with a single binary value: 0 = false, 1 = true

#### Numbers as Binary
A Number can be represented by translating the binary into it's equivalent number 0 = 0, 1 = 1, 10 = 2, 11 = 3

#### Strings as Binary
The initial binary objects for strings was actually a Char (character). A list of characters could be converted into a single string. Which was simplified in later programming languages
For example: The word "test" would be formatted in binary as `01110100 01100101 01110011 01110100`. Essentially stored as 4 separate character values

### What happens if things get larger?
One of the main problems of early programming is what occurrs when storing a variable in the computer's memory. Early computers only had 4mb of memory. As such, many computer engineers had to be very aware of memory limitations when creating programs.

What would happen persay if you needed to store the number 1,222,333,444,555,666,777. This is a hypotehtical situation, but large numbers are actually quite difficult to work with as they are being stored in a `binary` format as described above. 

The main issues to solve are:
- How do I prevent the numbers from maxing out my memory on the computer
- How do I store the variables in a way on the computer so that they don't accidently overwrite each other when adjusted. (for example, let's say I take the number above and times it by 10. That would increase the side of the binary, which could accidently affect other variables)

Engineers developed a `typing` system for variables. Which would put fixed limits on data types.

For example, in Rust there is the i32, and i64 types.
The max number ranges of these are repsectively:
- i32: -2147483647 -> 2147483647
- i64: -9223372036854775808 -> 9223372036854775808

This prevents variable overflow adjusting and affecting other variables.


## Why do some programming languages not have types? Like Javascript?
Programming languages can be broken down into:
- strongly typed languages
- loosely typed languages
- non-typed languages

We will ignore loosely typed languages for now. 

### Strongly typed languages
Rust and Go are strongly typed languages. They require you to set a type for each variable when you set it. You are not allowed to deviate from the type you set the variable at.

```
var myString string = "test"
myString = 0
```
The above code will throw an compiler error, stating that you are unable to assign a number to a string.

### Non Typed languages
Non typed languages like Javascript operate under different rules that typed languages. This is mostly possible because they are not languages that are phsyically attached to computer memory. They maintain their own memory storage at a higher level.

```
let myString = "test";
myString = 0;
```

The above code would be allowed in Javascript. As a programming language, it does not care about types, and is non opinionated on the subject.

## In summary
Primative types were established in order to:
- Support basic values in computing programs and a consistent interface for users to operate on
- Prevent variables from overflowing and affecting other variables
- Facilitate the process of converting user data into binary