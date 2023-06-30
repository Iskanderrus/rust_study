fn main(){ 
    let my_name = "Alex"; 
    match my_name { 
        "Alex" => println!("{:?} is my name", my_name), 
        "Anna" => println!("{:?} is name of my wife", my_name), 
        "Yarik" => println!("{:?} is name of my son", my_name), 
        "Leah" => println!("{:?} is name of my daughter", my_name), 
        _ => println!("{my_name}'s name is not in my family's name list"),
    }
}