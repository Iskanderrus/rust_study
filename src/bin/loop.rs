fn main() {
    let mut counter = 0;
    loop { 
        counter += 1;
        println!("This is loop {:?}", counter); 
        if counter == 10 {
            println!("All loops are done!");
            break;
        }
    }
}