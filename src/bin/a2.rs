// Use a function to sum two numbers. 
fn num_sum(a: i32, b: i32) -> i32 {
    a + b
}

// Use a function to print the resulted sum. 
fn print_sum(result: i32) { 
    println!("{:?}", result);
}


fn main() {
    let result = num_sum(5, 2); 
    print_sum(result);
}