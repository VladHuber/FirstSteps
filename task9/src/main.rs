use std::io;

use rand::Rng;

fn main() {
    println!("Симулятор крипто трейдера");
    let mut money = 1000.0;
    let mut btc = 0.0;
    let mut btc_price = 100.0;
    let mut day = 1;
    loop{
        println!("Текущая цена BTC: {}", btc_price);
        println!("Баланс: {}$\nBTC: {}\nЦена BTC: {}$", money, btc, btc_price);
        println!("1. Купить, 2. Продать, 3. Ждать");
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        match command.trim(){
            "1" => {
                btc += money / btc_price;
                money = 0.0;
            },
            "2" => {
                money += btc * btc_price;
                btc = 0.0;
            }
            "3" => {

            }
            _ => {
                break;
            }
        }
        if money as u32 >= 1001{
            println!("Вы выиграли");
            break;
        }
        let rng_number = rand::random_range(1..=10);
        let bts_last_price = btc_price;
        btc_price += rng_number as f64;
        day += 1;
        println!("День {}", day);
        println!("Цена изменилась {} -> {}", bts_last_price, btc_price);
        
    }
}
