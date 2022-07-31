use std::io;
use std::io::prelude::*;

fn main() {
    let mut a = String::new();
    let mut b = String::new();
    let r: i64;

    print!("Entre o valor de <A>: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut a).unwrap();
    let a = a.trim().parse::<i64>().unwrap();
    
    print!("Entre o valor de <B>: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut b).unwrap();
    let b = b.trim().parse::<i64>().unwrap();

    r = if a > b {a - b} else {b - a};

    println!("Diferença do maior pelo menor valor: {}", r);

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();
}
