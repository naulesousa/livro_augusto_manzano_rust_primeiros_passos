use std::io;
use std::io::prelude::*;

fn main() {
    let mut n1 = String::new();
    let mut n2 = String::new();
    let mut n3 = String::new();
    let mut n4 = String::new();
    let arithmetic_average: f64;

    print!("Entre o valor da primeira nota: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut n1).unwrap();
    let n1 = n1.trim().parse::<f64>().unwrap();

    
    print!("Entre o valor da primeira nota: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut n2).unwrap();
    let n2 = n2.trim().parse::<f64>().unwrap();


    print!("Entre o valor da primeira nota: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut n3).unwrap();
    let n3 = n3.trim().parse::<f64>().unwrap();


    print!("Entre o valor da primeira nota: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut n4).unwrap();
    let n4 = n4.trim().parse::<f64>().unwrap();

    arithmetic_average = (n1 + n2 + n3 + n4) / 4.0;

    if arithmetic_average >= 5.0 {
        println!("Aprovado com média - Média = {}", arithmetic_average);
    } else {
        println!("Reprovado com média - Média = {}", arithmetic_average);
    }

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();
}
