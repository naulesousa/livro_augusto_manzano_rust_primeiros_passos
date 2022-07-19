use std::io;
use std::io::prelude::*;

fn main() {
    let mut fahrenheit = String::new();
    let f: f64;
    let c: f64;

    print!("Entre com a temperatura em Fahrenheit: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut fahrenheit).unwrap();
    f = fahrenheit.trim().parse::<f64>().unwrap();

    c = ((f - 32.0) * 5.0) / 9.0;

    println!("Temperatura em graus Celcius: {}", c);

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();
}
