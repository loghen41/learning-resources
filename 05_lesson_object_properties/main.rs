fn main() {
	// Exercise 1. As a recap to last week, Create a function that takes a string, concatinates the word `Hello` to the beginning, and then return the string

	println!("1. Exercise one answer below -------------------");

	// Objects
	// Objects are a programming language syntax for defining clusters of properties together
	// They are referred to objects, because they are meant to reflect a single item, with the properties acting as attributes of that item

	// The Alias of the object - VideoGame
	struct VideoGame {
		name:         String, // A string property specific to the name of the object
		eerp_rating:  String, // A string property specific to the EERP rating of the object
		cost:         i32,    // An integer property for the cost of the video game
		is_available: bool   // A boolean value for whether or not the game is available
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
	struct Car {}

	// Exercise 2. Declare a completely empty object
	println!("2. Exercise two values below ----------------");

	// Object Properties
	// Properties can be connect to an object. they can be of any data type: string, bool, int, float, event other objects
	// Properties are defined in the same place we define the object alias, inside the {}
	// For example

	struct Garage {
		height: i32,
		width:  i32,
		depth:  i32,
		car:    Car
	}

	// Exercise 3. Construct an object with three properties
	println!("3. Exercise three values below ----------------");

	// Object instances
	// In lesson 1, we learned to establish primative variables: string, bool, int, float
	// Each variable was it's own unique thing.
	// For example

	let my_first_int = 2;
	let my_second_int = 5;

	println!("first int: {} second int: {}", my_first_int, my_second_int);

	// We've covered that these are separate variables.
	// This can also be done with objects.. for example

	let my_first_garage = Garage{
		height: 12,
		width:  24,
		depth:  30,
		car:    Car{}
	};

	let my_second_garage = Garage{
		height: 8,
		width:  12,
		depth:  15,
		car:    Car{}
	};

	println!("first garage {} {} {}", my_first_garage.height, my_first_garage.width, my_first_garage.depth);
	println!("second garage {} {} {}", my_second_garage.height, my_second_garage.width, my_second_garage.depth);

	// Each object is it's own value, and it's own variable. We call this an `instance` of an object
	// Exercise 4. Create two separate instances of your object from exercise three
	// Print out your objects
	println!("4. Exercise four values below ----------------");


	// Object Construction
	// You can absolutely instantiate an object in the literal sense like we did above
	// However, Rust allows for building a constructor to simplify the process of creating objects.
	// A constructor is a function that creates an instance of an object.
	// This is done through an `implementation` of the `new` method As shown by `impl ObjectName { fn new() -> ObjectName }

	impl Garage { // Put impl Object {} name at the start of the constructor

		// Define a function called `new` within the {} of the implementation and have it return the Object
		// Place variables for parameters in the function for all of the properties on the object
		fn new(height: i32, width: i32, depth: i32, car: Car) -> Garage {
			// Create the object the same way that you did before in code, but use the variables to fill in the fields
			return Garage { 
				height: height,
				width:  width,
				depth:  depth,
				car:    car
			}
		}
	}

	// Exercise 5. Create a constructor for your your object from exercise three
	println!("5. Exercise five values below ----------------");

	// Once the constructor is created, you can now utilize the constructor to create objects for you using the ::new() function
	// For example

	let my_third_garage = Garage::new(20, 40, 50, Car{});
	println!("garage {} {} {}", my_third_garage.height, my_third_garage.width, my_third_garage.depth);

	// Exercise 6. Create a new version of your object using the constructor you established in exercise five. print out the values of the object
	println!("6. Exercise six values below ----------------");

	// In summary
	// Objects allow for clustering of properties into a single item. They are broken down into three parts
	// - Object alias for the name of the object
	// - Object properties inside of the object
	// - When created as a variable, we refer to it as an instance of an object
	// - Constructors make it easier to create objects using a single function
}
