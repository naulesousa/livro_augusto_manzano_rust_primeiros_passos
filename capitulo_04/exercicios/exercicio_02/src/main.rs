use std::io;
use std::io::prelude::*;

fn main() {
    let mut soma: i32 = 0;
    for i in 1..101 {
        soma += i;
    }
    println!("{}", soma);

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();
}
