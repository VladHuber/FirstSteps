use std::io;
use std::thread;
use std::time::Duration;

use rand::Rng;

fn main() {
    println!("RPG Battle");
    println!("Нажмите чтобы начать >:))");
    io::stdin()
        .read_line(&mut String::new())
        .expect("Не нажали, не дожали :(");
    let mut hp_hero = 100.0;
    let mut hp_enemy = 100.0;
    let crit_chance = 0.2;
    let awade_chance = 0.1;
    
    let handle = thread::spawn(move || {
        loop {
            let attack_hero = rand::thread_rng().gen_range(10..=25) as f64;
            let attack_enemy = rand::thread_rng().gen_range(10..=30) as f64;

            let awade_rnd = rand::random::<f64>();
           
           
            println!("Ты ударил на {}", attack_hero);
            hp_enemy = hp_enemy - attack_hero;

            if awade_chance > awade_rnd {
                println!("Ты уклонился на {}", attack_hero);
            }else{
                let crit_rnd = rand::random::<f64>();
                if crit_rnd < crit_chance{
                    println!("Крит ударил на {}", (attack_enemy * 2.0));
                    hp_hero = hp_hero - (attack_enemy * 2.0);
                }else{
                    println!("Монстр ударил на {}", attack_hero);
                    hp_hero = hp_hero - attack_enemy;
                }
                
            }
           

            println!("HP героя: {} --- HP монстра: {}", hp_hero, hp_enemy);
            if hp_hero <= 0.0 || hp_enemy <= 0.0 {
                break;
            }
            thread::sleep(Duration::from_secs(1));
        }
        (hp_hero, hp_enemy)
    });
    let (hp_hero, hp_enemy) = handle.join().unwrap();
    println!("{} - {}", hp_hero, hp_enemy);
    if hp_hero < 0.0 {
        println!("Герой проиграл! \n HP монстра: {}", hp_enemy);
    }
    if hp_enemy < 0.0 {
        println!("Герой выиграл! \n оставшиеся HP: {}", hp_hero);
    }
}

