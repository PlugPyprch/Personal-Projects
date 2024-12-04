// fn main(){
//     let x = 5; // Immutable by default
//     let mut y = 10; // Mutable variable
//     y+=1;
//     println!("x: {}, y:{}", x, y);
// }

// Scalar: i32, f64, char, bool
// Compound: tuples, arrays

// let tup:(i32, f64, char) = (500, 6.4, 'a);
// let arr:[i32, 3] = [1, 2, 3];

fn main(){
    greet("Alices");
    println!("5 + 3 = {}", add(5, 3));
}

fn greet(name: &str){
    println!("Hello, {}",name);
}

fn add(a: i32, b: i32) -> i32{
    a+b
}