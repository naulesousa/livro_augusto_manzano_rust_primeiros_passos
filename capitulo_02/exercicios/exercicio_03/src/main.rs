use std::io;
use std::io::prelude::*;

fn main() {
    let mut a_string = String::new();
    let mut b_string = String::new();

    print!("Entre a variável A: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut a_string).unwrap();
    let mut a: i32 = a_string.trim().parse::<i32>().unwrap();

    print!("Entre a variável B: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut b_string).unwrap();
    let mut b: i32 = b_string.trim().parse::<i32>().unwrap();

    let c = a;
    a = b;
    b = c; 

    println!("Variáveis A = {} e B = {} com valores trocados", a, b);

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();
}
