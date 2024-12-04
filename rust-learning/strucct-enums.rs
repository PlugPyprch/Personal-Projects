// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle{
//     fn area(&self) -> u32{
//         self.width * self.height
//     }
// }

// fn main(){
//     let rect = Rectangle { width: 30, height:50};
//     println!("Area: {}", rect.area());
// }

//Enum

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn move_robot(direction: Direction){
    match direction {
        Direction::Up => println!("Moving up"),
        Direction::Down => println!("Moving down"),
        Direction::Left => println!("Moving left"),
        Direction::Right => println!("Moving right"),
    }
}

fn main(){
    move_robot(Direction::Up);
}