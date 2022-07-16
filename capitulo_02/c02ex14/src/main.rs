/// main.rs

/* Projeto ......: c02ex14
 * Autor (es) ...: Augusto Manzano/Blacksmith
 * Data .........: 16/07/2022
 * Versao .......: 1.0
 * Revisao ......: 1607/2022
 * */

use std::*;
use std::io::prelude::*;

fn main() {
/// Trecho com apresentação da faixa mínima de máxima de inteiros de 8 bits

    println!("i8 ...: de {:20} até {:20}", i8::MIN, i8::MAX);
    println!("u8 ...: de {:20} até {:20}", u8::MIN, u8::MAX);

    println!();
    print!("Tecle <Enter> para encerrar..."); // Acione apenas <Enter>
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();
}
