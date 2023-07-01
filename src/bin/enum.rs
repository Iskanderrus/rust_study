enum Direction {
    South, 
    North, 
    West, 
    East
}

fn main() {
    let go = Direction::North; 
    match go {
        Direction::East => println!("go East"), 
        Direction::North => println!("go North"), 
        Direction::South => println!("go South"), 
        Direction::West => println!("go West"),
    }
}