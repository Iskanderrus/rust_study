enum Colors {
    White, 
    Black, 
    Blue,
    Green, 
    Yellow, 
    Red, 
    Orange,
    Purple, 
}

fn print_car_color(my_car_color: Colors) {
    match my_car_color {
        Colors::Black => println!("My car is black"),
        Colors::Blue => println!("My car color is blue"), 
        Colors::Red => println!("My car color is red"), 
        Colors::White => println!("My car color is white"), 
        Colors::Orange => println!("My car color is orange"), 
        Colors::Green => println!("My car color is green"),
        Colors::Purple => println!("My car color is purple"),
        Colors::Yellow => println!("My car color is yellow")
    }
}



fn main() {
    let my_car_color = Colors::Red; 
    print_car_color(my_car_color);
}