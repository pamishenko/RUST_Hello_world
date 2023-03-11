use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Угадай число!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("Секретное число равно {}", secret_number);
loop {
    println!("Пожалуйста? введите свою догадку.");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Не получилось прочитать строку");

    let guess: u32 = guess.trim().parse()
        .expect("input digit, please");

    println!("Вы загадали: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("small"),
        Ordering::Greater => println!("big"),
        Ordering::Equal => {
            println!("\x1b[32msuccess\x1b[0m");
            break;
        },
    }
}
}
