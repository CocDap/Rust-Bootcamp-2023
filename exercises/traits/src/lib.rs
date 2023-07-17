// Exercise 1
// Fill in the two impl blocks to make the code work.
// Make it compile
// Run tests
trait Hello {
    fn say_hi(&self) -> String {
        String::from("hi")
    }

    fn say_something(&self) -> String;
}

//TODO 
struct Student {}
impl Hello for Student {
}
//TODO
struct Teacher {}
impl Hello for Teacher {
}


// Exercise 2
// Make it compile in unit test for exercise 2
// Hint: use #[derive]  for struct Point 
// Run tests
struct Point {
    x: i32,
    y: i32,
}


// Exercise 3
// Make it compile 
// Implement `fn sum` with trait bound in two ways.
// Run tests
// Hint: Trait Bound
fn sum<T>(x: T, y: T) -> T {
    x + y
}


// Exercise 4
// Fix errors and implement
// Hint: Static Dispatch and Dynamic Dispatch
// Run tests
trait Foo {
    fn method(&self) -> String;
}

impl Foo for u8 {
    fn method(&self) -> String { format!("u8: {}", *self) }
}

impl Foo for String {
    fn method(&self) -> String { format!("string: {}", *self) }
}

// IMPLEMENT below with generics and parameters
fn static_dispatch(x) {
    todo!()
}

// Implement below with trait objects and parameters
fn dynamic_dispatch(x) {
    todo!()
}

// Exercise 5 
// Fix errors and fill in the blanks 
// Run tests
// Hint: &dyn and Box<dyn>
trait Draw {
    fn draw(&self) -> String;
}

impl Draw for u8 {
    fn draw(&self) -> String {
        format!("u8: {}", *self)
    }
}

impl Draw for f64 {
    fn draw(&self) -> String {
        format!("f64: {}", *self)
    }
}

fn draw_with_box(x: Box<dyn Draw>) {
    x.draw();
}

fn draw_with_ref(x: __) {
    x.draw();
}

// Exercise 6
// Fix errors and implement 
// Run tests
// Hint: Associated Type

trait Container {
    type Item;
    fn insert(&mut self, item: Self::Item);
    fn remove(&mut self) -> Option<Self::Item>;
    fn is_empty(&self) -> bool;
}

struct Stack {
    items: Vec<u8>,
}

//TODO implement Container for Stack



#[cfg(test)]
mod tests {
    use super::*;

    // Test for exercise 2

    #[test]
    fn exercise1_should_work() {
        let s = Student {};
        assert_eq!(s.say_hi(), "hi");
        assert_eq!(s.say_something(), "I'm a good student");
    
        let t = Teacher {};
        assert_eq!(t.say_hi(), "Hi, I'm your new teacher");
        assert_eq!(t.say_something(), "I'm not a bad teacher");   
    }

    #[test]
    fn exercise2_should_work() {
        let point1 = Point { x: 1, y: 2 };
        let point2 = Point { x: 1, y: 2 };
        let point3 = Point { x: 3, y: 4 };

        assert_eq!(point1, point2);
        assert_ne!(point1, point3);
    }

    #[test]
    fn exercise3_should_work() {
        assert_eq!(sum(1, 2), 3);
    }

    #[test]
    fn exercise4_should_work() {
        let x = 5u8;
        let y = "Hello".to_string();
    
        static_dispatch(x);
        dynamic_dispatch(&y); 
    }

    #[test]
    fn exercise5_should_work() {
        let x = 1.1f64;
        let y = 8u8;
    
        // Draw x.
        draw_with_box(__);
    
        // Draw y.
        draw_with_ref(&y);
    }

    #[test]
    fn exercise6_should_work(){
        let mut stack: Stack<u8> = Stack { items: Vec::new() };
        assert!(stack.is_empty());
        stack.insert(1);
        stack.insert(2);
        stack.insert(3);
        assert!(!stack.is_empty());
        assert_eq!(stack.remove(), Some(3));
        assert_eq!(stack.remove(), Some(2));
        assert_eq!(stack.remove(), Some(1));
        assert_eq!(stack.remove(), None);
        assert!(stack.is_empty());
    }

}
