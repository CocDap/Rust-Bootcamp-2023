use std::ops::{Add, Div, Mul, Rem, Sub};

fn main() {
    let circle = Circle { radius: 10.0 };

    println!("Circle :{:?}", circle.area());
    println!("Circle draw :{:?}", circle.draw());

    let mut counter = Counter { x: 0.0 };
    println!("Counter next:{:?}", counter.next());
    println!("Counter next:{:?}", counter.next());
    let mut counter_a = CounterAssociated { x: 0 };
    println!("Counter next:{:?}", counter_a.next());
    println!("Counter next:{:?}", counter_a.next());

    let mut vec: Vec<u32> = vec![1, 2, 3, 4];

    let res = vec.next().unwrap();
    println!("res:{}", res);

    println!("Create shape:{:?}", create_shape());
    println!("Create shape 2 :{:?}", create_shape_2());

    let circle1 = Circle { radius: 10.0 };
    let rec = Rectangle {
        width: 10.0,
        height: 10.0,
    };

    let test: Vec<u8> = vec![1, 23];
    let tri = Triangle {};
    let shapes: Vec<Box<dyn Shape>> = vec![Box::new(circle1), Box::new(rec)];
    // làm như thế nào để có thể lưu được 2 object là circle1 và rec
    // biến trait thành 1 kiểu dữ liệu ( object)
    // trait object
    for shape in shapes {
        let res = shape.area();
    }

    // trait object làm nhiệm vụ gì
    // static dispatch
    // static : tĩnh
    // dispatch : execute 1 hàm

    // cách sử dụng static dispatch
    draw_static(&circle);
    draw_dynamic(&circle);
    // trait bound 


    let res = String::from(ErrorCustom::NotFound);
    println!("res:{}",res);

    let res2: String= ErrorCustom::NotFound.into();
    println!("res:{}",res2);

    let res3 = String::from_fake(Box::new(ErrorCustom::NotFound));
    println!("res:{}",res3);

    let student = Student {
        grade: 10,
        name: "VBI".to_string()
    };

    println!("student:{}",student);
}

// static dispatch
// áp dụng trait bound

fn draw_static<T: Drawable>(shape: &T) {
    shape.draw();
}

fn draw_static_other(shape: &impl Drawable) {
    shape.draw();
}

// ở compile time, biết đc  object đã xác định như
// Circle, Rectangle

// fn draw_static_circle(shape: &Circle) {
//     shape.draw();
// }

// fn draw_static_rectangle(shape: &Rectangle) {
//     shape.draw();
// }

// dynamic dispatch

fn draw_dynamic(shape: &dyn Drawable) {
    shape.draw();
}
// chạy runtime

struct Triangle {}
#[derive(Debug)]
struct Circle {
    radius: f64,
}

#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

trait Drawable {
    fn draw(&self);
}

trait Shape: Drawable + std::fmt::Debug {
    fn area(&self) -> f64;
}
impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing a circle.");
    }

    // fn area(&self) -> f64 {
    //     std::f64::consts::PI * self.radius * self.radius
    // }
}

impl Drawable for Rectangle {
    fn draw(&self) {
        println!("Drawing a rectangle.");
    }

    // fn area(&self) -> f64 {
    //     self.width * self.height
    // }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

fn create_shape() -> impl Shape { //trả về kiểu dữ liệu (struct Circle) mà trait (Shape) có impl cái trait (Shape) đó
    Circle { radius: 10.0 } //hay thằng Shape có impl cho thằng Circle
}
// Box : smart pointer
// kiểu như String, Vec::new()
fn create_shape_2() -> Box<dyn Shape> {
    Box::new(Circle { radius: 10.0 })
}

fn foo() -> &'static str {
    "Hello"
}

// cần xác nhận
// fn create_shape_3() -> &'static dyn Shape {
//     &Box::new(Circle {radius: 10.0})

// }
// super traits

// Associated type

// generic type

// generic type
pub trait Iterator<T> {
    fn next(&mut self) -> T;
}
// cho mình một gợi ý là thay vì mình sử dụng x là u32 thì mình
// sử dụng generic
pub struct Counter<T> {
    x: T,
}

impl<T: std::ops::Add<f64, Output = T> + Copy> Iterator<T> for Counter<T> {
    fn next(&mut self) -> T {
        self.x = self.x + 1.0;
        self.x
    }
}

trait Number:
    Copy
    + Clone
    + Eq
    + PartialEq
    + Ord
    + PartialOrd
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
{
}

pub struct CounterAssociated {
    x: u32,
}

// associated type
pub trait IteratorA {
    type Item;
    fn next(&mut self) -> Self::Item;
}

impl IteratorA for CounterAssociated {
    type Item = u32;

    fn next(&mut self) -> u32 {
        self.x = self.x + 1;
        self.x
    }
}

// cần discuss
trait IteratorVec {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

impl IteratorVec for Vec<u32> {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        Some(10)
    }
}

struct Container(i32, i32);

// interface -> shared behaviour cho các kiểu dữ liệu khác nhau
trait Contains {
    type A;
    type B;
    fn contains(&self, _: &Self::A, _: &Self::B) -> bool; // Explicitly requires `A` and `B`.
    fn first(&self) -> i32; // Doesn't explicitly require `A` or `B`.
    fn last(&self) -> i32; // Doesn't explicitly require `A` or `B`.
}

impl Contains for Container {
    type A = i32;
    type B = i32;
    // True if the numbers stored are equal.
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }

    // Grab the first number.
    fn first(&self) -> i32 {
        self.0
    }

    // Grab the last number.
    fn last(&self) -> i32 {
        self.1
    }
}

// `C` contains `A` and `B`. In light of that, having to express `A` and
// `B` again is a nuisance.
fn difference<C: Contains>(container: &C) -> i32 {
    container.last() - container.first()
}

//static dispatch
// dynamic dispatch

// static : trait bound
// dynamic : dyn

trait Foo {
    fn method(&self) -> String;
}

impl Foo for u8 {
    fn method(&self) -> String {
        format!("u8: {}", *self)
    }
}

impl Foo for String {
    fn method(&self) -> String {
        format!("string: {}", *self)
    }
}

// IMPLEMENT below with generics and parameters
// trait bound
fn static_dispatch(x: impl Foo) {
    let _ = x.method();
}
// Implement below with trait objects and parameters
// dyn
fn dynamic_dispatch(x: Box<dyn Foo>) {
    todo!()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exercise4_should_work() {
        let x = 5u8;
        let y = "Hello".to_string();

        static_dispatch(x);
        dynamic_dispatch(Box::new(y));
    }
}

// #[derive(Debug)]
enum ErrorCustom {
    NotFound,
    FailToCreate,
}

impl std::error::Error for ErrorCustom {

}

impl std::fmt::Display for ErrorCustom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self)
    }
}

impl std::fmt::Debug for ErrorCustom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound => write!(f, "NotFound"),
            Self::FailToCreate => write!(f, "FailToCreate"),
        }
    }
}


// làm sao mình có thể chuyển dc kiểu dạng từng phần tử của enum sang String

// Enum -> String
impl From<ErrorCustom> for String {
    fn from(value: ErrorCustom) -> Self {
        match value {
            ErrorCustom::NotFound => "not found".to_string(),
            ErrorCustom::FailToCreate => "fail to create".to_string(),
        }
    }
}

trait FromFake<T> {
    fn from_fake(value: T) -> Self;

}

impl FromFake<Box<dyn std::error::Error>> for String {
    fn from_fake(value: Box<dyn std::error::Error>) -> Self {
        value.to_string()
    }
}

struct Student {
    grade: u8,
    name: String

}

impl std::fmt::Display for Student {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //write!(f, "{},{}", self.grade, self.name)
        format!("{},{}", self.grade, self.name).fmt(f)
    }
}



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

fn draw_with_ref(x: &dyn Draw) {
    x.draw();
}


// overview 
// phân biệt static dispatch và dynamic dispatch : dyn
// cách implement 1 trait nào đó (từ thư viện ) hoặc tự customized


// có 2 cách để mình đưa ra 1 bài tập tổng hợp đầy đủ 
// cách 1:  mình làm sẵn hết , mình đưa ra unit test -> mn viết logic
// cách 2 : chỉ đưa ra yêu cầu và gợi ý -> tự mn implement
