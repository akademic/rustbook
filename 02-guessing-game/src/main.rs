extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    println!("Угадай число!");

    let secret = rand::thread_rng().gen_range(1,101);

    //println!("Загаданное число: {}", secret);

    loop {
        println!("Введи число:");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .ok()
            .expect("Не могу прочитать");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Надо вводить число");
                continue;
            }
        };

        println!("Ты ввёл: {}", guess);

        match guess.cmp(&secret) {
            Ordering::Less => println!("Маловато будет"),
            Ordering::Greater => println!("Давай поменьше"),
            Ordering::Equal => {
                println!("Угадал, чертяка");
                break;
            }
        }
    }

}
