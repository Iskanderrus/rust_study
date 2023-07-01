fn main() {
    let mut i = 4; 
    loop {
        println!("{:?}", i); 
        i -= 1; 
        if i == 0{
            break;
        }
    }
}