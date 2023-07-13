#![allow(dead_code)]

// fn main() {
//     println!("Hello, world!");
//     //print_u8(10u64 as u8);// cast
//     //print_u64(10u64);
//     print_generic(10u8);
//     print_generic(20u64);
//     print_generic("Hello");
//     print_generic("Hello".to_string());
//     print_generic(vec![1, 2, 3]);
//     // monomorphization
//     //
//     returns_reference();
//     let x = Some(32);
//     let y: Option<u32> = None;

//     print_generic::<&str>("Hello");
//     print_generic::<u64>(10u64);

//     let point = Point::<f64> { x: 1.0, y: 2.0 };
//     //let point1: Point<u64> = Point::new(10, 20);
//     let point1 = Point::<i32>::new(10, 20);
//     let point2 = Point::new(10, 20);

//     let x = Some(5);
//     let y= Some(5.0f64);

// }

struct Point1 {
    x: u32,
    y: u32,
}

impl Point1 {
    fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

struct Point<T> {
    x: T,
    y: T,
}
impl<AB> Point<AB> {
    fn new(x: AB, y: AB) -> Self {
        Self { x, y }
    }
}

// generic type: kiểu dữ liệu chung, tổng hợp
// type :
// primitive
// collections
// trait
// generic: chung , tổng hợp
// place holder

// fn print_u8(input: u8) {
//     println!("Input:{}",input);

// }

// fn print_u64(input: u64) {
//     println!("Input:{}",input);

// }
// turbofish
// Đinhj nghĩa generic type bằng  ký tự in hoa
fn print_generic<T: std::fmt::Debug>(input: T) {
    println!("Input:{:?}", input);
}

//Option

struct PointTwo<T, U> {
    x: T,
    y: U,
}

fn returns_reference<'a>() -> &'a str {
    let my_string = String::from("I am a string");
    //&my_string // ⚠️
    "Hello"
}
// đánh dấu lifetime

// Trait: đặc tính

// interface

// pub struct Car{
//     category: String
// }

// impl Car {
//     fn get_category(&self){
//         println!("Category:{}",self.category);

//     }

// }

trait Car {
    fn get_category(&self) -> String;
    fn speed(&self) -> u32;

    // general
    //
    //
}

// định nghĩa
struct Sedan {}
// impl Sedan{

// }

impl Car for Sedan {
    fn get_category(&self) -> String {
        "Bốn bánh".to_string()
    }

    fn speed(&self) -> u32 {
        100
    }
}

struct Coupe {}
impl Car for Coupe {
    fn get_category(&self) -> String {
        "Xe mui trần".to_string()
    }

    fn speed(&self) -> u32 {
        200
    }
}
// nhận xét
//    sử dụng các đặc tính chung để mô tả cho object cụ thể nào đó
//  tính riêng biệt của mỗi object dựa trên đặc tính chung

struct MPV {}

struct SUV {}

fn main() {
    // let vios = Sedan {};
    // let speed_vios = vios.speed();

    // let bwm = Coupe {};

    // let speed_bwm = bwm.speed();

    let circle = Circle { radius: 10.0 };
    let rec = Rectangle {
        width: 10.0,
        height: 20.0,
    };

    println!("Circle area:{}", circle.area());
    println!("Rec area:{}", rec.area());
    let a = get_area(circle);
    let b = get_area(rec);
    println!("Circle area with trait bound:{}", a);
    let tri = Triangle {};
    //let c = get_area(tri);

    // cách dùng đối với Square<T>
    let square: Square<f64> = Square { x: 10.0 };
    println!("square area:{}", square.area());

    let c = get_area(square);
}

struct Triangle {}

struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

trait Drawable {
    fn draw(&self);
    fn area(&self) -> f64;
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing a circle.");
    }

    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Drawable for Rectangle {
    fn draw(&self) {
        println!("Drawing a rectangle.");
    }

    fn area(&self) -> f64 {
        self.width * self.height
    }
}

// nhưng T ràng buộc bởi trait Drawable
// cách định nghĩa
// trait bound : generic type đang ràng buộc một số trait
// generic function sử dụng trait
fn get_area<T>(method: T) -> f64
where
    T: Drawable,
{
    method.area()
}

// fn get_area(method: impl Drawable) -> f64 {
//     method.area()
// }

// cách dùng

// generic struct

struct Square<T> {
    x: T, // primitive // string , Heap
}

// cộng nhiều trait gọi là
impl<T: std::ops::Mul<Output = f64> + Copy> Drawable for Square<T> {
    fn draw(&self) {
        println!("Draw square")
    }

    fn area(&self) -> f64 {
        self.x * self.x
    }
}

// impl std::ops::Mul for Square<f64> {
//     type Output = f64;

//     fn mul(self, rhs: Self) -> f64 {
//         let res = self.x * rhs.x;
//         res
//     }
// }

// fn create_shape() -> object mà trait Drawable có implement {
//     Circle { radius: 5.0 }
// }

fn create_shape() -> impl Drawable {
    //Circle { radius: 5.0 }
    // Triangle{}
    Square { x: 10f64 }
}


// fn create_shape_other<T: Drawable>() -> T {
//     T
// }

// Summary 
// Generic type
// life time
// trait
// định nghĩa trait
// impl trait 
// cách sử dụng
// tại sao sử dụng trait
// trait bound ( function with generic type + trait , struct generic + trait)
// return trait 


// Chú ý 
// kiểu dữ liệu phải đc implement