fn main() {
    println!("Hello, world!");
    let x: u8 = 130;
    let boo = false;

    let y = 10.5f32;
    let array: [f32; 4] = [10.0, 20.0, 10.0, 10.0];
    println!("array 1:{}, {}", array[0], array[2]);
    println!("array 4:{}", array[3]);
    let tuple1 = (10, 10);
    let tuple1 = (10, 10, 10.0, "s");
    println!("tuple :{}", tuple1.0);
    //println!("tuple :{}",tuple1.4);
    //let strage: u32;
    //println!("result :{}",strage);
    type A = u32;
    // name convention
    let mut x = 10;
    // mutable
    // immutable
    //let ho_dung = "10";
    x = x + 10;
    println!("result :{}", x);

    const PI: f64 = 3.14;
    const NUMBER_CASE: u32 = 1;

    let mut s = String::new();
    println!("string is empty :{}", s.is_empty());
    s.push('h');
    println!("string is empty :{}", s);

    let mut s = String::from("Hello world");
    println!("string i :{}", s);

    //std::mem::size_of::<char>();
    //cast
    //reference str
    let s2 = "Hello World";
    s = s + s2;
    println!("string is  :{}", s);

    println!("result is  :{}", &s2[0..5]);

    let mut s3 = "Hello World";
    //s3 = s3 + "Dung";
    //s3.push("ABC");
    // conversion &str -> String
    let conversion_string = "VBI".to_string();
    let conversion_str = &conversion_string;
    let conversion_str = &&&&conversion_string.as_str();

    let byte = conversion_string.as_bytes();
    println!("result is  :{:?}", byte);
    let conversion_str = &*conversion_string;

    let mut s: String = String::new();
    println!("{}", s);

    let mut s: &str = "Test";
    println!("{}", s);
    let s_for = format!("{}", "Hello VBI");

    let x = true;
    if x {
        println!("Hello");
    } else {
        println!("Bye");
    }

    //Pattern matching
    // match : switch case/ case

    match x {
        true => println!("Hello"),
        false => println!("Bye"),
    }
    // default match

    let number = 5;
    match number {
        _ => println!("Hello the other world"),
        5 => println!("Hello :{}", number),
        10 => println!("Hello :{}", number),
        5 => println!("Hello Invalid"),
    }
    let tuple1 = (10, 10);
    match tuple1 {
        (10, 10) | (20, 20) => match number {
            5 => println!("Hello :{}", number),
            10 => println!("Hello :{}", number),
            _ => println!("Hello Invalid"),
        },
        _ => todo!(),
    }

    let vec = vec![1, 23, 4, 5, 6];
    // for value in vec {

    //     println!("Value:{}",value);
    // }
    // for value in vec.iter() {
    //     println!("Value:{}",value);

    // }
    for index in 0..vec.len() {}
    let max = vec.iter().max().unwrap();
    println!("MAx:{}", max);
    // Phân biệt giữa for bình thường và iter() hoặc iter_mut(), into_iter();
    function_x();
    function_y(String::from("XYZ"));
    let x = function_z(String::from("XYZ"));

}

fn function_x() {
    println!("ABC");
}
fn function_y(s: String) {
    println!("ABC:{}", s);
}
fn function_z(s: String) -> String {
    s
    
}

// fn function_y(x: &str) {
//     println!("ABC:{}", x);
// }

// fn function_x(x: &str) -> String  {
//     println!("ABC");
// }
