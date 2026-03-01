use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Digite um número");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is {secret_number}");

    loop {

    let mut number = String::new();

    io::stdin()
    .read_line(&mut number)
    .expect("Digite um número válido");

    let number: i32 = match number.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    println!("Você digitou {}", number);

    match number.cmp(&secret_number) {
        Ordering::Less => println!("Muito pequeno"),
        Ordering::Greater => println!("Muito grande"),
        Ordering::Equal => {
         println!("You win");
         break;
        }
    }
}
}
