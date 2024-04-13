package main

import "fmt"

func main() {
	// Exercise 1. As a recap to last week, Create a function that takes a string, concatinates the word `Hello` to the beginning, and then return the string

	fmt.Println("1. Exercise one answer below -------------------")

	// Objects
	// Objects are a programming language syntax for defining clusters of properties together
	// They are referred to objects, because they are meant to reflect a single item, with the properties acting as attributes of that item

	// The Alias of the object - VideoGame
	type VideoGame struct {
		Name        string // A string property specific to the name of the object
		EERPRating  string // A string property specific to the EERP rating of the object
		Cost        int    // An integer property for the cost of the video game
		IsAvailable bool   // A boolean value for whether or not the game is available
	}

	// To understand this in detail, we are going to break down objects into it's specific items
	// The object Alias/Name
	// The object properties
	// Object instances / construction

	// The Object Alias/Name
	// Objects are declared specifically with the syntax type alias struct {}. Everything else can be completely ignored.
	// The ojbect won't do anything, but it is possible to build a object that way.
	// for example

	// The Object alias is Car
	type Car struct{}

	// Exercise 2. Declare a completely empty object
	fmt.Println("2. Exercise two values below ----------------")

	// Object Properties
	// Properties can be connect to an object. they can be of any data type: string, bool, int, float, event other objects
	// Properties are defined in the same place we define the object alias, inside the {}
	// For example

	type Garage struct {
		Height int
		Width  int
		Depth  int
		Car    Car
	}

	// Exercise 3. Construct an object with three properties
	fmt.Println("3. Exercise three values below ----------------")

	// Object instances
	// In lesson 1, we learned to establish primative variables: string, bool, int, float
	// Each variable was it's own unique thing.
	// For example

	myFirstInt := 2
	mySecondInt := 5

	fmt.Printf("first int: %v second int: %v \n", myFirstInt, mySecondInt)

	// We've covered that these are separate variables.
	// This can also be done with objects.. for example

	myFirstGarage := Garage{
		Height: 12,
		Width:  24,
		Depth:  30,
	}

	mySecondGarage := Garage{
		Height: 8,
		Width:  12,
		Depth:  15,
	}

	fmt.Printf("first garage: %v second garage: %v \n", myFirstGarage, mySecondGarage)

	// Each object is it's own value, and it's own variable. We call this an `instance` of an object

	// Exercise 4. Create two separate instances of your object from exercise three
	// Print out your objects
	fmt.Println("4. Exercise four values below ----------------")

	// In summary
	// Objects allow for clustering of properties into a single item. They are broken down into three parts
	// - Object alias for the name of the object
	// - Object properties inside of the object
	// - When created as a variable, we refer to it as an instance of an object
}
