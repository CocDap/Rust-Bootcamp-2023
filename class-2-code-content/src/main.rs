fn main() {
    // sử dụng closure
    let x = |y: &str| -> String { y.to_string() };
    let res = x("VBI");
    println!("Res:{}", res);

    // for , iter, into_iter , iter_mut
    let vec1 = vec![1, 2, 3, 4, 5];

    // for value in vec1 {
    //     println!("Value:{}",value);
    // }

    let vec2 = vec![1, 2, 3, 4, 5];
    // vec2.iter().enumerate().for_each(|x|{
    //     println!("Index:{}, value:{}",x.0,x.1)

    // });
    let res: Vec<i32> = vec2.iter().map(|x| x * 2).collect();
    println!("res:{:?}", res);

    let mut vec_new = vec![];

    for value in vec1 {
        vec_new.push(value * 2);
    }
    println!("Cach 2:{:?}", vec_new);

    let vec3: Vec<i32> = vec![];
    let res = vec3.iter().max();
    println!("res:{:?}", res);

    assert_eq!(count_char_occurrences("Rust is fun", 'u'), 2);
    assert_eq!(count_char_occurrences("Mississippi", 's'), 4);

    // mutable
    let s1 = String::from("Hello world");
    // s1 đang sở hữu dữ liệu "Hello world"

    //immutable
    //let s2 = &s1[..];
    // gabage collector

    let s2 = s1;
    //s1 gán cho s2 và đồng thời s2  sở hữu dữ liệu "Hello world"
    // theo nguyên tắc ownership của Rust
    // 1 biến chỉ sở hữu 1 giá trị tương ứng
    // xoá s1: drop s1
    // s1 move value to s2
    println!("S2:{}", s2);
    //println!("S1:{}",s1);

    // Rust primitive : u32,u64, ..
    // Collections: vector, string,

    // Stack và Heap

    // Biến
    // là tên để lưu giá trị -> lưu ở 1 vùng nhớ nào đó
    // Memory Management

    // Pointer : trỏ địa chỉ của biến đó

    //let s3 = s1;
    // vi phạm nguyên tắc của ownership;
    // owernship: sở hữu

    //let s3 = s1;

    // ownership
    // {} : scope

    let x = 10;
    // x : global
    {
        //y: local
        let y = "Hello";
        println!("Y:{}", y);
    }
    println!("y:{}", x);
    // Rust: out of scope -> value drop

    function_a();
    //print!("z:{}",z);

    let x = 42;
    let y = 43;
    let var1 = &x;
    // pointer là 1 con trỏ -> trỏ tới biến 
    // mang địa chỉ 
    println!("var:{:p}",&var1);

    let s3 = String::from("Hello ");
    
    let s4 = s3;

    let s5 = s4;
    //println!("s3 address:{:?}",s3.as_ptr());
    //println!("s4 address:{:?}",s4.as_ptr());

    //println!("S3:{}",s3);
    // tham chiếu 
    // Stack và Heap 
    // Stack : địa chỉ ô nhớ để lưu giá trị cố định 
    // Heap: ô nhớ  động 

    // compile time 
    // runtime 

    // Primitive: default là stack -> biết size 
    // collections (string, vec) có bỏ vào stack dc hay ko ?
    // ko biết size ở compile time 
    // lưu ở HEAP 

    let mut s6 =String::from("Hell");
    println!("len:{}, capacity:{}", s6.len(),s6.capacity());
    s6.push_str("o");
    println!("len:{}, capacity:{}", s6.len(),s6.capacity());
    s6.push_str("World");
    println!("len:{}, capacity:{}", s6.len(),s6.capacity());

    let mut s7 =String::from("Hell");

    // let s8 =s7.clone();

    // println!("s7 address:{:?}",s7.as_ptr());
    // println!("s8 address:{:?}",s8.as_ptr());

    //println!("S7:{}",s7);
    //s7.push_str("WOrld");
    //let s9 = &s7;
    // shared reference
    // immutable 

    //s9.push_str("World");
    //let s10 = &s7;
    //let s11 = &s7;
    // có thể shared reference nhiều lần 


    // mutable reference 
    // có thể thay đổi giá trị
    // nhưng owner phải share quyền thay đổi mut 

    // let s13 = &s7;
    // println!("S13:{}",s13);
    // let s14 = &s7;
    // let s15 = &s7;
    // println!("S14:{}",s14);
    let s16 = &mut s7;
    let s17 = &s7;

    println!("s16:{}",s16);
    



    // s12.push_str("World");
    // println!("S7:{}",s7);
    // let s13 = &s7;
    // let s14 = &s13;






    //println!("s9:{}",s9);


    let s = String::from("hello");

    change(s);
    //println!("s2:{}",s2);
    println!("s:{}",s);
}

fn change(mut some_string:  String) {
    
    some_string.push_str(", world");
}
fn function_a() {
    println!("Hello");
    let z = 10;
}

fn count_char_occurrences(string: &str, ch: char) -> usize {
    let res = string.chars().into_iter().filter(|c| c == &ch).count();
    res
}

// borrow checker 




