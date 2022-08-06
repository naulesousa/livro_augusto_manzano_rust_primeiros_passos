use std::io;
use std::io::prelude::*;

fn main() {
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();
    let mut d = String::new();

    print!("Entre o valor de <A>: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut a).unwrap();
    let a = a.trim().parse::<i64>().unwrap();
    
    print!("Entre o valor de <B>: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut b).unwrap();
    let b = b.trim().parse::<i64>().unwrap();

    print!("Entre o valor de <C>: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut c).unwrap();
    let c = c.trim().parse::<i64>().unwrap();

    print!("Entre o valor de <D>: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut d).unwrap();
    let d = d.trim().parse::<i64>().unwrap();
    
    if let a % 2 = 0 && a % 3 = 0 {
        println!("{}", a)
    }
    if let b % 2 = 0 && b % 3 = 0 {
        println!("{}", b)
    }
    if let c % 2 = 0 && c % 3 = 0 {
        println!("{}", c)
    }
    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();
}
