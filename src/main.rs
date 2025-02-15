use pomodoro::start_timer;
use std::io;

fn main() {
    loop {
        println!("--- Menu --- ");
        println!("1. Focus time @ 25mins");
        println!("2. Short break @ 5mins");
        println!("3. Long break @ 15mins");
        println!("4. Exit");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => {
                println!("Stay focued for 25m");
                start_timer(1500);
                break;
            }
            2 => {
                println!("Take a short break for 5m");
                start_timer(300);
                break;
            }
            3 => {
                println!("Relax a little for 15m");
                start_timer(900);
                break;
            }
            4 => {
                break;
            }
            _ => {
                println!("Invalid choice");
            }
        }
    }
}
