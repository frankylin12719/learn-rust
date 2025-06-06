// 标准库引入io
use rand::Rng;

use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // PART2
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");

    loop {
        // PART1
        println!("Please enter your guess.");

        let mut guess: String = String::new();

        io::stdin()
            // 读取输入行的值赋给guess
            .read_line(&mut guess)
            // 错误控制
            .expect("failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        //PART3
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Big!!"),
            Ordering::Greater => println!("Too Small!!"),
            Ordering::Equal => {
                println!("WIN!!");
                break;
            }
        }
    }
}
