// Rust code file covering various concepts

// Structs
struct Person {
    name: String,
    age: u32,
}

impl Person {
    // Methods
    fn new(name: String, age: u32) -> Self {
        Person { name, age }
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_age(&self) -> u32 {
        self.age
    }
}

// Enums
enum Language {
    Rust,
    Python,
    JavaScript,
}

// Traits
trait Greeting {
    fn greet(&self);
}

impl Greeting for Language {
    fn greet(&self) {
        match self {
            Language::Rust => println!("Hello from Rust!"),
            Language::Python => println!("Hello from Python!"),
            Language::JavaScript => println!("Hello from JavaScript!"),
        }
    }
}

// Generics
fn print_vector<T: std::fmt::Debug>(vector: &[T]) {
    for element in vector {
        println!("{:?}", element);
    }
}

// Error handling
fn divide(x: i32, y: i32) -> Result<i32, String> {
    if y == 0 {
        Err("Division by zero".to_string())
    } else {
        Ok(x / y)
    }
}

fn main() {
    // Variables and mutability
    let mut x = 5;
    println!("x: {}", x);
    x = 10;
    println!("x: {}", x);

    // Ownership and borrowing
    let name = String::from("Alice");
    let age = 25;
    let person = Person::new(name, age);
    println!("Name: {}", person.get_name());
    println!("Age: {}", person.get_age());

    // Control flow
    let number = 7;
    if number < 5 {
        println!("The number is less than 5");
    } else if number > 10 {
        println!("The number is greater than 10");
    } else {
        println!("The number is between 5 and 10");
    }

    // Loops
    let mut counter = 0;
    while counter < 5 {
        println!("Counter: {}", counter);
        counter += 1;
    }

    for number in 1..=5 {
        println!("Number: {}", number);
    }

    // Arrays and vectors
    let array = [1, 2, 3, 4, 5];
    println!("Array: {:?}", array);

    let vector = vec![1, 2, 3, 4, 5];
    print_vector(&vector);

    // Error handling
    match divide(10, 2) {
        Ok(result) => println!("Result: {}", result),
        Err(error) => println!("Error: {}", error),
    }

    // Enums and traits
    let lang = Language::Rust;
    lang.greet();
}
