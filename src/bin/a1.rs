
pub const MY_FIRST_NAME : &str = "Alexander"; 
pub const MY_LAST_NAME : &str = "Chasovskoy"; 


/// .
fn first_name() {
    println!("{:?}", MY_FIRST_NAME);
}

fn last_name() {
    println!("{:?}", MY_LAST_NAME);
}

fn name_printing() {
    println!("{MY_FIRST_NAME} {MY_LAST_NAME}");
}

pub fn main() {
    first_name();
    last_name();
    name_printing();
}