use std::io;
use std::io::prelude::*;

fn main() {
    let mut numero = String::new();
    let n: u64;
    let quadrado: u32;

    print!("Entre um número: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut numero).unwrap();
    n = numero.trim().parse::<u64>().unwrap();
    
    let numero: u32 = n.try_into().unwrap();

    quadrado = numero.pow(2);

    println!("N = {} N^2 = {}", numero, quadrado);

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();
}
