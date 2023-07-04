fn main() {
    // standard tuple assignment: 
    let coord = (2, 3); 
    println!("Coordinates are: {:?}, {:?}", coord.0, coord.1); 

    // tuple destructing - easier to access the values and keep code clean: 
    let (x, y) = (2 ,3); 
    println!("Coordinates are: {:?}, {:?}", x, y); 

    // if tuple is larger then 3 values, use struct instead
}