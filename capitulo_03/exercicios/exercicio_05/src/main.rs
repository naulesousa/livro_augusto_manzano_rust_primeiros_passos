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

    let is_div_a1: i64 = a % 2;
    let is_div_a2: i64 = a % 3;
    let is_div_b1: i64 = b % 2;
    let is_div_b2: i64 = b % 3;
    let is_div_c1: i64 = c % 2;
    let is_div_c2: i64 = c % 3;
    let is_div_d1: i64 = d % 2;
    let is_div_d2: i64 = d % 3;
    
    if is_div_a1 == 0 && is_div_a2 ==0 {
        println!("{}", a)
    }
    
    if is_div_b1 == 0 && is_div_b2 == 0 {
        println!("{}", b)
    }
    
    if is_div_c1 == 0 && is_div_c2 == 0 {
        println!("{}", c)
    }
    
    if is_div_d1 == 0 && is_div_d2 == 0 {
        println!("{}", d)
    }

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();
}
