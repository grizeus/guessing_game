extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Відгадайте число!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Введіть, будь ласка, число: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Не вдалося прочитати рядок :( ");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Ваша здогадка: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Замало!"),
            Ordering::Greater => println!("Забагато!"),
            Ordering::Equal => {
                println!("Ви вгадали!");
                break;
            }
        }
    }
}
