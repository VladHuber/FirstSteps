use std::io;
use std::io::Write;
use std::thread;
use std::time::Duration;

fn main() {
    start_game();
}

fn start_game() {
    let asscessing = String::from("ACCESSING DATABASE...");
    display_str(asscessing);
    display_str(String::from("PASSWORD FOUND"));
    println!();
    // clear_screen();
    fill_progress_bar();
    fill_progress_bar_v2();
}
fn display_str(word: String) {
    for c in word.chars() {
        print!("{c}");
        io::stdout().flush().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    print!("\r");
    println!();
}

// fn clear_screen() {
//     print!("\x1B[2J\x1B[H");
//     io::stdout().flush().unwrap();
// }
fn fill_progress_bar() {
    for i in 0..=100 {
        print!("\rProgress {i}%");
        io::stdout().flush().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}
fn fill_progress_bar_v2() {
    const WIDTH: usize = 30;

    for percent in 0..=100 {
        let filled = percent * WIDTH / 100;

        let bar = format!(
            "[{}{}] {}%",
            "=".repeat(filled),
            " ".repeat(WIDTH - filled),
            percent
        );

        print!("\r{bar}");
        io::stdout().flush().unwrap();

        thread::sleep(Duration::from_millis(50));
    }
}
