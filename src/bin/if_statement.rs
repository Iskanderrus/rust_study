fn main() {
    let age = 13; 
    if age >= 21 && age < 90 {
        println!("ok to purchase a beer");
    } else if age <= 21 {
        println!("you are too young to buy a beer");
    } else {
        println!("it's too late to drink a beer");
    }
}