struct GroceryItem {
    stock: i32, 
    price: f64,
}

fn main() {
    let apple = GroceryItem {
        stock: 200,
        price: 1.25,
    };
    println!("There are {:?} KG of apples in stock available.", apple.stock);
    println!("Current apples price is: {:?} TL", apple.price);
}