use std::io;
use std::fmt::Display;

fn main() {
    println!("Список дел (to-do list)");
    println!("\n");
    println!("--------------------------");
    println!("Напишите список дел");
    let mut dela: Vec<String> = Vec::new();
   
    loop{
        let mut delo = String::new();
        io::stdin().read_line(&mut delo).expect("Nope");
        match delo.trim() {
            "x" => {
                break;
            }
            _ => {
                
            }
            
        }
        dela.push(delo.trim().to_string());
        if dela.len() > 3{
            break;
        }
            
    }
    
    println!("{:?}",dela);

}
