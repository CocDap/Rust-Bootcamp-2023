// Exercise 1
// Fix the error
// Make it compile
// Run test
struct Person {
    name: String,
    age: u8,
    hobby: String
}
fn exercise1() -> Person {
    let age = 30;
    // Hobby = Rust 
    let p = Person {
        name: String::from("sunface"),
        age,
        hobby: String::from("Rust")
    };

    p
}

// Exercise 2
// Fix the error
// Make it compile
// Run test

// Define the struct
struct Agent  {
    name: String,
    age: u32,
}

// Implementation of methods for the Person struct
impl Agent {
    // Create a new Person instance
    fn new(name: String, age: u32) -> Agent {
        Agent { name, age }
    }

    // Get the name of the person
    fn get_name(&self) -> &str {
        todo!()
    }

    // Get the age of the person
    fn get_age(&self) -> u32 {
        todo!()
    }
}

// Exercise 3
// Fix the error
// Make it compile
// Run test
struct Calculator {
    value: i32,
}

impl Calculator {
    fn new() -> Self {
        Calculator { value: 0 }
    }

    fn add(&self, num: i32) {
        self.value += num;
    }

    fn subtract(mut self, num: i32) {
        self.value -= num;
    }
    fn clear(self) {
        self.value = 0;
    }

    fn get_value(self) -> i32 {
        self.value
    }
}

// Exercise 4
// Make it compile
#[derive(Debug)]
struct User {
    first: String,
    last: String,
    age: u32,
}

fn exercise4() {
    let u1 = User {
        first: String::from("John"),
        last: String::from("Doe"),
        age: 22,
    };

    let u2 = User {
        first: String::from("Mary"),
        ..u1
        
    };

    println!("user: {:#?}", u1);

}

// Exercise 5
// Make it compile
struct Foo {
    str_val: String,
    int_val: i32,
}

fn exercise5() {
    let mut foos = Vec::new();
    foos.push(Foo {
        str_val: "ten".to_string(),
        int_val: 10,
    });
    foos.push(Foo {
        str_val: "twenty".to_string(),
        int_val: 20,
    });

    
    let moved = foos[0];

    
    let moved_field = foos[0].str_val;
}

// Exercise 6
// Structs contain data, but can also have logic. In this exercise we have
// defined the Package struct and we want to test some logic attached to it.
// Make the code compile and the tests pass!

#[derive(Debug)]
struct Package {
    sender_country: String,
    recipient_country: String,
    weight_in_grams: i32,
}

impl Package {
    fn new(sender_country: String, recipient_country: String, weight_in_grams: i32) -> Package {
        if weight_in_grams <= 0 {
            panic!("Can not ship a weightless package.")
        } else {
            Package {
                sender_country,
                recipient_country,
                weight_in_grams,
            }
        }
    }

    fn is_international(&self) -> ??? {
        // Something goes here...
    }

    fn get_fees(&self, cents_per_gram: i32) -> ??? {
        // Something goes here...
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test for exercise 1
    #[test]
    fn exercise1_should_work() {
        let p = exercise1();

        let p_expectation = Person {
            name: String::from("sunface"),
            age: 30,
            hobby:String::from("Rust") 
        };
        assert_eq!(p, p_expectation);
        
    }

    // Test for exercise 2
    #[test]
    fn exercise2_should_work() {
        // Create a new Person instance
        let agent = Agent::new(String::from("John"), 30);

        // Test the get_name method
        assert_eq!(agent.get_name(), "John");

        // Test the get_age method
        assert_eq!(agent.get_age(), 30);
    }

    // Test for exercise 3
    #[test]
    fn exercise3_should_work() {
        let mut calculator = Calculator::new();
        calculator.add(5);
        assert_eq!(calculator.get_value(), 5);

        calculator.subtract(2);
        assert_eq!(calculator.get_value(), 3);

        calculator.clear();
        assert_eq!(calculator.get_value(), 0);

    }


    // Test for exercise 6
    #[test]
    #[should_panic]
    fn fail_creating_weightless_package() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Austria");

        Package::new(sender_country, recipient_country, -2210);
    }

    // Test for exercise 6
    #[test]
    fn create_international_package() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Russia");

        let package = Package::new(sender_country, recipient_country, 1200);

        assert!(package.is_international());
    }

    // Test for exercise 6
    #[test]
    fn create_local_package() {
        let sender_country = String::from("Canada");
        let recipient_country = sender_country.clone();

        let package = Package::new(sender_country, recipient_country, 1200);

        assert!(!package.is_international());
    }
    // Test for exercise 6
    #[test]
    fn calculate_transport_fees() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Spain");

        let cents_per_gram = 3;

        let package = Package::new(sender_country, recipient_country, 1500);

        assert_eq!(package.get_fees(cents_per_gram), 4500);
        assert_eq!(package.get_fees(cents_per_gram * 2), 9000);
    }

}