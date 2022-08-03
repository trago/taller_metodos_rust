use std::io;
use rand::Rng;

fn main() {
    let number: u8 = rand::thread_rng().gen::<u8>() % 10 + 1;

    println!("Adivina en que numero pienso del 1 al 10)");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("No pude leer la linea de entrada");

    let my_guess = input.trim().parse::<u8>().unwrap();

    if my_guess == number{
        println!("Adivinaste!!")
    }else {
        println!("No le atinaste, mi numero es {number}")
    }
}
