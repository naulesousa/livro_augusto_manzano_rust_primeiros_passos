use std::io;
use std::io::prelude::*;

fn main() {
    let mut quadrado: u32;
    for i in 15..201 {
        quadrado = i * i;
        println!("{}", quadrado);
    }
    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();
}
