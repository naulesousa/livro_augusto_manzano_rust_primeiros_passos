use std::io;
use std::io::prelude::*;

fn main() {
    println!("abs(-3.5) .....: {}", -3.5f64.abs());

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();
}
