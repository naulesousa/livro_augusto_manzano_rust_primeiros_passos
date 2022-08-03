use std::io;
use std::io::prelude::*;

fn main() {
    let mut n1 = String::new();
    let mut n2 = String::new();
    let mut n3 = String::new();
    let mut n4 = String::new();
    let mut ne = String::new();
    let md1: f64;
    let md2: f64;

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

    md1 = (n1 + n2 + n3 + n4) / 4.0;

    if md1 >= 7.0 {
        println!("Aprovado com média: {}", md1); 
    } else {
        println!("Entre o valor da nota de exame: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut ne).unwrap();
        let ne = ne.trim().parse::<f64>().unwrap();

        md2 = (md1 + ne) / 2.0;

        if md2 >= 5.0 {
            println!("Aprovado em exame com média: {}", md2);
        } else {
            println!("Reprovado com média: {}", md2);
        }
    }

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();
}
