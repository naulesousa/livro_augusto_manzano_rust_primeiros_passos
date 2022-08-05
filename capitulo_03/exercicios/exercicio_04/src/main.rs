use std::io;
use std::io::prelude::*;

fn main() {
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();

    print!("Entre o valor A: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut a).unwrap();
    let a = a.trim().parse::<u32>().unwrap();
    
    print!("Entre o valor B: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut b).unwrap();
    let b = b.trim().parse::<u32>().unwrap();
    
    print!("Entre o valor C: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut c).unwrap();
    let c = c.trim().parse::<u32>().unwrap();
    
    if (a < b) && (b < c) {
        println!("{}, {}, {}", a, b, c);
    } else if (a < c) && (c < b) {
        println!("{}, {}, {}", a, c, b);
    } else if (b < a) && (a < c) {
          println!("{}, {}, {}", b, a, c);
        } else if (b < c) && (c < a) {
            println!("{}, {}, {}", b, c, a);
        } else if (c < a) && (a < b) {
            println!("{}, {}, {}", c, a, b);
        } else {
            println!("{}, {}, {}", c, b, a);
        }

        println!();
        print!("Tecle <Enter> para encerrar...");
        io::stdout().flush().unwrap();
        io::stdin().read(&mut [0u8]);
}

