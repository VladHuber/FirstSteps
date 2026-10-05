use std::io;

// use rand::Rng;

enum Rarity{
    Common = 59, Rare = 10, Legendary = 3
}

fn main() {
    println!("Нажми чтобы открыть сундук");
    io::stdin().read_line(&mut String::new()).unwrap();
    open_box();
}
fn open_box(){
    println!("Выпал предмет!");
    // let chance_rng = rand::thread_rng().gen_range(0..=100);
    let chance_rng = 99;
    let event = match chance_rng{
        0..79 => Rarity::Common,
        80..95 => Rarity::Rare,
        96..100 => Rarity::Legendary,
        _ => unreachable!()
    };
    match event{
        Rarity::Common => {
            println!("Выпал обычный предмет")
        },
        Rarity::Rare => {
            println!("Выпал редкий предмет")
        },
        Rarity::Legendary => {
            println!("Выпал легендарный предмет")
        }
    }
}
