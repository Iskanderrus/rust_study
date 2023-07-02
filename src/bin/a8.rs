enum DrinkFlavor {
    Apple,
    Orange, 
    Vishne, 
}

enum Brand {
    Dobry,
}
struct Drink {
    flavor: DrinkFlavor, 
    volume: f64,
    brand: Brand,
}

fn product_printing(drink: Drink) {

    match drink.brand {
        Brand::Dobry => println!("Drink brand is: Dobry"),
    };
     
    match drink.flavor {
        DrinkFlavor::Apple => println!("Drink flavor is: Apple"),
        DrinkFlavor::Orange => println!("Drink flavor is: Orange"), 
        DrinkFlavor::Vishne => println!("Drink flavor is: Vishne"), 
    };
    println!("Drink volume: {:?} FLOZ\n", drink.volume); 
}

fn main() {
    let sok_dobry_apple = Drink {
        brand: Brand::Dobry,
        flavor: DrinkFlavor::Apple, 
        volume: 2.5,
    };
    let sok_dobry_orange = Drink {
        brand: Brand::Dobry,
        flavor: DrinkFlavor::Orange, 
        volume: 2.5,
    };
    let sok_dobry_vishne = Drink {
        brand: Brand::Dobry,       
        flavor: DrinkFlavor::Vishne, 
        volume: 2.5,
    };
    product_printing(sok_dobry_apple);
    product_printing(sok_dobry_orange);
    product_printing(sok_dobry_vishne);
}