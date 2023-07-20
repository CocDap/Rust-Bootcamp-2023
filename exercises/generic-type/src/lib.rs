// Exercise 1 
// Implement struct Point to make it work.
// Make it compile
fn exercise1() {
    let integer = Position { x: 5, y: 10 };
    let float = Position { x: 1.0, y: 4.0 };
}



// Exercise 2
// Modify this struct to make the code work
// Make it compile
struct Point<T> {
    x: T,
    y: T,
}

fn exercise2() {
    // DON'T modify this code.
    let p = Point{x: 5, y : "hello".to_string()};
}



// Exercise 3
// Make it compile
// Add generic for Val to make the code work, DON'T modify the code in `main`.
struct Val {
    val: f64,
}

impl Val {
    fn value(&self) -> &f64 {
        &self.val
    }
}


fn exercise3() {
    let x = Val{ val: 3.0 };
    let y = Val{ val: "hello".to_string()};
    println!("{}, {}", x.value(), y.value());
}

// Exercise 4
// Find the maximum value in a collection
// Make it compile
// Implementing logic
// Run tests

fn find_max<T>(collection: &[T]) -> Option<&T> {
    todo!()
}

// Exercise 5 
// Reverse the elements in a collection
// Make it compile 
// Run tests 
fn reverse_collection<T>(collection: &[T]) {
    todo!()
}


// Exercise 6
// Function to check if a collection contains a specific value
fn contains_value<T>(collection: &[T], value: &T) -> bool {
    todo!()
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    // Test for exercise 4
    #[test]
    fn test_find_max_with_numbers() {
        let numbers = vec![1, 5, 3, 8, 2];
        assert_eq!(find_max(&numbers), Some(&8));
    }

    // Test for exercise 4
    #[test]
    fn test_find_max_with_strings() {
        let strings = vec!["apple", "banana", "cherry", "durian"];
        assert_eq!(find_max(&strings), Some(&"durian"));
    }

    // Test for exercise 4
    #[test]
    fn test_find_max_with_empty_collection() {
        let empty: Vec<i32> = Vec::new();
        assert_eq!(find_max(&empty), None);
    }

    // Test for exercise 5
    #[test]
    fn test_reverse_collection_with_numbers() {
        let mut numbers = vec![1, 2, 3, 4, 5];
        reverse_collection(&mut numbers);
        assert_eq!(numbers, vec![5, 4, 3, 2, 1]);
    }

    // Test for exercise 5
    #[test]
    fn test_reverse_collection_with_strings() {
        let mut strings = vec!["apple", "banana", "cherry", "durian"];
        reverse_collection(&mut strings);
        assert_eq!(strings, vec!["durian", "cherry", "banana", "apple"]);
    }

    // Test for exercise 5
    #[test]
    fn test_reverse_collection_with_empty_collection() {
        let mut empty: Vec<i32> = Vec::new();
        reverse_collection(&mut empty);
        assert_eq!(empty, Vec::<i32>::new());
    }

    // Test for exercise 6
    #[test]
    fn test_contains_value_with_numbers() {
        let numbers = vec![1, 2, 3, 4, 5];
        assert_eq!(contains_value(&numbers, &3), true);
        assert_eq!(contains_value(&numbers, &6), false);
    }

    // Test for exercise 6
    #[test]
    fn test_contains_value_with_strings() {
        let strings = vec!["apple", "banana", "cherry", "durian"];
        assert_eq!(contains_value(&strings, &"banana"), true);
        assert_eq!(contains_value(&strings, &"grape"), false);
    }

    // Test for exercise 6
    #[test]
    fn test_contains_value_with_empty_collection() {
        let empty: Vec<i32> = Vec::new();
        assert_eq!(contains_value(&empty, &5), false);
    }

}
