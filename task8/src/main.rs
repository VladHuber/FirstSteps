use std::io;

#[derive(Debug)]
enum ItemType {
    Weapon, Shield, Armor
}
struct Item{
    name : String,
    item_type : ItemType
}

use std::fmt;

impl fmt::Display for ItemType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ItemType::Weapon => write!(f, "Оружие"),
            ItemType::Shield => write!(f, "Щит"),
            ItemType::Armor => write!(f, "Броня"),
        }
    }
}
fn main() {
    let mut inventory = Vec::<Item>::new();
    loop{
        println!("Команды:\n a - добавить \n r - удалить \n d - отобразить \n x - выйти");
        let mut command = String::new();

        io::stdin().read_line(&mut command).unwrap();
        match command.trim(){
            "a" => {
                println!("Введите имя");
                let mut item = String::new();
                io::stdin().read_line(&mut item).unwrap();
                println!("Введите тип");
                let mut item_type_string = String::new();
                io::stdin().read_line(&mut item_type_string).unwrap();
                let mut item_type = ItemType::Armor;
                match item_type_string.to_lowercase().trim(){
                    "weapon" => {
                        item_type = ItemType::Weapon
                    }
                    "shield" => item_type = ItemType::Shield,
                    _ => {}
                }
                inventory.push(Item { name: item.trim().to_string() , item_type: item_type});
            }
            "r" =>{
                println!("Что удалить?");

                display_inventory(&inventory);
                let mut item_delete_index = String::new();
                io::stdin().read_line(&mut item_delete_index).unwrap();
                let number = match item_delete_index.trim().parse::<usize>() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Это не число");
                        return;
                    }
                };
                match inventory.get(number){
                    Some(_item) => {
                        inventory.remove(number);
                        println!("Предмет удален");
                    },
                    None => {
                        println!("Предмет не найден");
                    }
                }
                
                
            }
            "d" => {
                display_inventory(&inventory);
            }
            "x" =>{
                break;
            }
            _ => {
                
            }
        }

    }
}

fn display_inventory(inventory : &Vec<Item>){
    for (key, item) in inventory.iter().enumerate(){
        println!("[{}] - {} - {}", key, item.name, item.item_type);
    }
}
