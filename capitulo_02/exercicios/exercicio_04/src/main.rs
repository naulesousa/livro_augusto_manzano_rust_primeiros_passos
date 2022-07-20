use std::io;
use std::io::prelude::*;

fn main() {
    let mut comprimento = String::new();
    let mut largura = String::new();
    let mut altura = String::new();
    let c: i32;
    let l: i32;
    let a: i32;
    let volume: i32;

    println!("Entre os valores solicitados abaixo para o cálculo do volume de uma caixa retangular.");

    print!("Entre o valor do comprimento...: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut comprimento).unwrap();
    c = comprimento.trim().parse::<i32>().unwrap();

    print!("Entre o valor da largura.......: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut largura).unwrap();
    l = largura.trim().parse::<i32>().unwrap();

    print!("Entre o valor da altura........: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut altura).unwrap();
    a = altura.trim().parse::<i32>().unwrap();

    volume = c * l * a;

    println!("Volume ......................: {} ", volume);

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();
}
