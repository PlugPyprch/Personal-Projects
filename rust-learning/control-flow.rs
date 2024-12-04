// fn main(){
//     let number = 7;
//     if number % 2 == 0{
//         println!("Even");
//     } else{
//         println!("Odd");
//     }
// }

// Loop
fn main(){
    let mut counter = 0;

    loop {
        if counter == 5{
            break;
        }
        counter +=1;
        println!("Counter: {}", counter);
    }

    for i in 1..4{
        println!("i: {}", i);
    }
}